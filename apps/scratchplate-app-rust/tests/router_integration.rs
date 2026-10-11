use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::header::{AUTHORIZATION, WWW_AUTHENTICATE};
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use scratchplate_app_rust::adapter::outbound::config::ConfigWelcomeTemplateAdapter;
use scratchplate_app_rust::application::service::GreetUserService;
use scratchplate_app_rust::build_main_router;
use scratchplate_app_rust::infrastructure::security::{JwtClaims, JwtVerifier, RealmAccessClaims, StaticKeySource};
use scratchplate_app_rust::WebState;
use serde_json::Value;
use tower::ServiceExt;

const ISSUER: &str = "http://issuer.example/realms/scratchplate";
const SECRET: &[u8] = b"scratchplate-test-secret";

fn test_router() -> axum::Router {
    let templates = Arc::new(ConfigWelcomeTemplateAdapter::new(
        "Welcome {username}! You are signed in as an administrator: you have full access to the platform.",
        "Welcome {username}! You are signed in as a regular user.",
    ));
    let greet_user = Arc::new(GreetUserService::new(templates));
    let verifier = Arc::new(JwtVerifier::new(ISSUER, Arc::new(StaticKeySource::new_hs256(SECRET))));
    let state = Arc::new(WebState::new(greet_user, verifier, true));
    build_main_router(state, true)
}

#[tokio::test]
async fn welcomes_admin() {
    let response = test_router()
        .oneshot(
            Request::builder()
                .uri("/api/v1/welcome")
                .header(AUTHORIZATION, format!("Bearer {}", token("admin", &["admin"], 3600, ISSUER, SECRET)))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(body["username"], "admin");
    assert_eq!(body["role"], "admin");
}

#[tokio::test]
async fn welcomes_user() {
    let response = test_router()
        .oneshot(
            Request::builder()
                .uri("/api/v1/welcome")
                .header(AUTHORIZATION, format!("Bearer {}", token("user", &["user"], 3600, ISSUER, SECRET)))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(body["message"], "Welcome user! You are signed in as a regular user.");
}

#[tokio::test]
async fn admin_wins_when_user_has_both_roles() {
    let response = test_router()
        .oneshot(
            Request::builder()
                .uri("/api/v1/welcome")
                .header(AUTHORIZATION, format!("Bearer {}", token("root", &["user", "ADMIN"], 3600, ISSUER, SECRET)))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = json_body(response).await;
    assert_eq!(body["role"], "admin");
}

#[tokio::test]
async fn rejects_missing_token() {
    let response = test_router()
        .oneshot(Request::builder().uri("/api/v1/welcome").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(response.headers().get(WWW_AUTHENTICATE).unwrap(), "Bearer");
}

#[tokio::test]
async fn rejects_bad_signature() {
    let response = test_router()
        .oneshot(
            Request::builder()
                .uri("/api/v1/welcome")
                .header(AUTHORIZATION, format!("Bearer {}", token("admin", &["admin"], 3600, ISSUER, b"wrong-secret")))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(response.headers().get(WWW_AUTHENTICATE).unwrap(), "Bearer");
}

#[tokio::test]
async fn rejects_expired_token() {
    let response = test_router()
        .oneshot(
            Request::builder()
                .uri("/api/v1/welcome")
                .header(AUTHORIZATION, format!("Bearer {}", token("admin", &["admin"], -3600, ISSUER, SECRET)))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn rejects_wrong_issuer() {
    let response = test_router()
        .oneshot(
            Request::builder()
                .uri("/api/v1/welcome")
                .header(AUTHORIZATION, format!("Bearer {}", token("admin", &["admin"], 3600, "http://wrong-issuer", SECRET)))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn forbids_authenticated_user_without_supported_role() {
    let response = test_router()
        .oneshot(
            Request::builder()
                .uri("/api/v1/welcome")
                .header(AUTHORIZATION, format!("Bearer {}", token("guest", &["offline_access"], 3600, ISSUER, SECRET)))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert!(body.is_empty());
}

#[tokio::test]
async fn denies_other_paths_for_anonymous_and_authenticated_callers() {
    let anonymous = test_router()
        .oneshot(Request::builder().uri("/anything").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(anonymous.status(), StatusCode::UNAUTHORIZED);

    let authenticated = test_router()
        .oneshot(
            Request::builder()
                .uri("/anything")
                .header(AUTHORIZATION, format!("Bearer {}", token("admin", &["admin"], 3600, ISSUER, SECRET)))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(authenticated.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn exposes_health_endpoint() {
    let response = test_router()
        .oneshot(
            Request::builder()
                .uri("/actuator/health/readiness")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(body["status"], "UP");
}

async fn json_body(response: axum::response::Response) -> Value {
    let body = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body).unwrap()
}

fn token(username: &str, roles: &[&str], exp_offset_seconds: i64, issuer: &str, secret: &[u8]) -> String {
    let exp = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
        + exp_offset_seconds) as usize;

    let claims = JwtClaims {
        iss: issuer.to_owned(),
        sub: format!("id-{username}"),
        preferred_username: Some(username.to_owned()),
        exp,
        realm_access: RealmAccessClaims {
            roles: roles.iter().map(|role| (*role).to_owned()).collect(),
        },
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret),
    )
    .unwrap()
}
