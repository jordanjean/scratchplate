use std::sync::Arc;

use axum::body::Body;
use axum::extract::{OriginalUri, Request, State};
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE, WWW_AUTHENTICATE};
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, Uri};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use serde_json::json;
use tracing::warn;

use crate::application::port_in::GreetUserUseCase;
use crate::domain::{MissingRoleError, Role, UserIdentity};
use crate::infrastructure::security::{JwtClaims, JwtVerifier};

#[derive(Clone)]
pub struct WebState {
    greet_user: Arc<dyn GreetUserUseCase>,
    jwt_verifier: Arc<JwtVerifier>,
    skip_auth_for_actuator: bool,
}

impl WebState {
    pub fn new(
        greet_user: Arc<dyn GreetUserUseCase>,
        jwt_verifier: Arc<JwtVerifier>,
        skip_auth_for_actuator: bool,
    ) -> Self {
        Self {
            greet_user,
            jwt_verifier,
            skip_auth_for_actuator,
        }
    }
}

#[derive(Debug, Clone)]
enum AuthContext {
    Anonymous,
    Authenticated(UserIdentity),
}

#[derive(Debug, Serialize)]
struct WelcomeResponse<'a> {
    message: &'a str,
    username: &'a str,
    role: &'a str,
}

pub fn build_main_router(state: Arc<WebState>, expose_actuator: bool) -> Router {
    let router = Router::new()
        .route("/api/v1/welcome", get(welcome_handler))
        .fallback(main_fallback)
        .layer(middleware::from_fn_with_state(state.clone(), authentication_middleware))
        .with_state(state);

    if expose_actuator {
        router
            .route("/actuator/health", get(health_handler))
            .route("/actuator/health/liveness", get(health_handler))
            .route("/actuator/health/readiness", get(health_handler))
            .route("/actuator/info", get(info_handler))
    } else {
        router
    }
}

pub fn build_management_router() -> Router {
    Router::new()
        .route("/actuator/health", get(health_handler))
        .route("/actuator/health/liveness", get(health_handler))
        .route("/actuator/health/readiness", get(health_handler))
        .route("/actuator/info", get(info_handler))
        .fallback(actuator_fallback)
}

async fn authentication_middleware(
    State(state): State<Arc<WebState>>,
    mut request: Request,
    next: Next,
) -> Response {
    if state.skip_auth_for_actuator && request.uri().path().starts_with("/actuator") {
        return next.run(request).await;
    }

    let auth_context = match bearer_token(request.headers()) {
        Some(token) => match state.jwt_verifier.verify(token).await {
            Ok(claims) => AuthContext::Authenticated(claims_to_user_identity(&claims)),
            Err(error) => {
                warn!(error = %error, "request authentication failed");
                return unauthorized_response();
            }
        },
        None => AuthContext::Anonymous,
    };

    request.extensions_mut().insert(auth_context);
    next.run(request).await
}

async fn welcome_handler(State(state): State<Arc<WebState>>, request: Request) -> Response {
    match auth_context(&request) {
        AuthContext::Anonymous => unauthorized_response(),
        AuthContext::Authenticated(user) if !user.has_supported_role() => forbidden_empty_response(),
        AuthContext::Authenticated(user) => match state.greet_user.greet(&user) {
            Ok(welcome) => (
                StatusCode::OK,
                Json(WelcomeResponse {
                    message: welcome.text(),
                    username: welcome.username(),
                    role: welcome.role().as_str(),
                }),
            )
                .into_response(),
            Err(error) => missing_role_problem_response(&error),
        },
    }
}

async fn main_fallback(uri: OriginalUri, request: Request) -> Response {
    let path = uri.path();
    match (path, auth_context(&request)) {
        (path, AuthContext::Anonymous) if path.starts_with("/actuator") => forbidden_empty_response(),
        (path, AuthContext::Anonymous) if path.starts_with("/api/") || path == "/api" => unauthorized_response(),
        (path, AuthContext::Authenticated(user)) if path.starts_with("/api/") || path == "/api" => {
            if user.has_supported_role() {
                StatusCode::NOT_FOUND.into_response()
            } else {
                forbidden_empty_response()
            }
        }
        (_, AuthContext::Anonymous) => unauthorized_response(),
        (_, AuthContext::Authenticated(_)) => forbidden_empty_response(),
    }
}

async fn actuator_fallback(method: Method, uri: Uri) -> Response {
    let _ = (method, uri);
    forbidden_empty_response()
}

async fn health_handler() -> impl IntoResponse {
    Json(json!({ "status": "UP" }))
}

async fn info_handler() -> impl IntoResponse {
    Json(json!({}))
}

fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    let value = headers.get(AUTHORIZATION)?.to_str().ok()?;
    value.strip_prefix("Bearer ")
}

fn auth_context(request: &Request) -> AuthContext {
    request
        .extensions()
        .get::<AuthContext>()
        .cloned()
        .unwrap_or(AuthContext::Anonymous)
}

fn claims_to_user_identity(claims: &JwtClaims) -> UserIdentity {
    let username = claims
        .preferred_username
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(&claims.sub);

    let roles = claims
        .realm_access
        .roles
        .iter()
        .filter_map(|role| Role::from_name(role))
        .collect::<Vec<_>>();

    UserIdentity::new(username, roles).expect("validated claims always contain a username")
}

fn unauthorized_response() -> Response {
    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::UNAUTHORIZED;
    response.headers_mut().insert(
        WWW_AUTHENTICATE,
        HeaderValue::from_static("Bearer"),
    );
    response
}

fn forbidden_empty_response() -> Response {
    StatusCode::FORBIDDEN.into_response()
}

fn missing_role_problem_response(error: &MissingRoleError) -> Response {
    let mut response = Json(json!({
        "title": "Missing role",
        "status": StatusCode::FORBIDDEN.as_u16(),
        "detail": error.to_string(),
    }))
    .into_response();
    *response.status_mut() = StatusCode::FORBIDDEN;
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/problem+json"),
    );
    response
}

#[cfg(test)]
mod tests {
    use super::{claims_to_user_identity, missing_role_problem_response};
    use crate::domain::MissingRoleError;
    use crate::infrastructure::security::{JwtClaims, RealmAccessClaims};

    #[test]
    fn maps_roles_case_insensitively_and_prefers_username_claim() {
        let claims = JwtClaims {
            iss: "issuer".to_owned(),
            sub: "id-1".to_owned(),
            preferred_username: Some("admin".to_owned()),
            exp: 9_999_999_999,
            realm_access: RealmAccessClaims {
                roles: vec!["USER".to_owned(), "Admin".to_owned(), "ignored".to_owned()],
            },
        };

        let user = claims_to_user_identity(&claims);
        assert_eq!(user.username(), "admin");
        assert_eq!(user.primary_role(), Some(crate::domain::Role::Admin));
    }

    #[test]
    fn falls_back_to_subject_when_preferred_username_is_blank() {
        let claims = JwtClaims {
            iss: "issuer".to_owned(),
            sub: "subject-42".to_owned(),
            preferred_username: Some("  ".to_owned()),
            exp: 9_999_999_999,
            realm_access: RealmAccessClaims::default(),
        };

        let user = claims_to_user_identity(&claims);
        assert_eq!(user.username(), "subject-42");
    }

    #[tokio::test]
    async fn missing_role_error_maps_to_problem_json() {
        let response = missing_role_problem_response(&MissingRoleError::new("guest"));
        assert_eq!(response.status(), axum::http::StatusCode::FORBIDDEN);
        assert_eq!(
            response.headers().get(axum::http::header::CONTENT_TYPE).unwrap(),
            "application/problem+json"
        );
    }
}
