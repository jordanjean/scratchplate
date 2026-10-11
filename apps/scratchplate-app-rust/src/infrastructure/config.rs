#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub server_port: u16,
    pub management_server_port: u16,
    pub jwt_issuer_uri: String,
    pub jwt_jwk_set_uri: String,
    pub welcome_admin_template: String,
    pub welcome_user_template: String,
}

impl AppConfig {
    pub const DEFAULT_SERVER_PORT: u16 = 8080;
    pub const DEFAULT_ISSUER_URI: &'static str = "http://localhost:32000/realms/scratchplate";
    pub const DEFAULT_JWK_SET_URI: &'static str =
        "http://localhost:32000/realms/scratchplate/protocol/openid-connect/certs";
    pub const DEFAULT_ADMIN_TEMPLATE: &'static str =
        "Welcome {username}! You are signed in as an administrator: you have full access to the platform.";
    pub const DEFAULT_USER_TEMPLATE: &'static str =
        "Welcome {username}! You are signed in as a regular user.";

    pub fn from_env() -> Self {
        let server_port = read_port("SERVER_PORT").unwrap_or(Self::DEFAULT_SERVER_PORT);
        let management_server_port = read_port("MANAGEMENT_SERVER_PORT").unwrap_or(server_port);

        Self {
            server_port,
            management_server_port,
            jwt_issuer_uri: read_env("JWT_ISSUER_URI")
                .unwrap_or_else(|| Self::DEFAULT_ISSUER_URI.to_owned()),
            jwt_jwk_set_uri: read_env("JWT_JWK_SET_URI")
                .unwrap_or_else(|| Self::DEFAULT_JWK_SET_URI.to_owned()),
            welcome_admin_template: read_env("SCRATCHPLATE_WELCOME_ADMIN_TEMPLATE")
                .unwrap_or_else(|| Self::DEFAULT_ADMIN_TEMPLATE.to_owned()),
            welcome_user_template: read_env("SCRATCHPLATE_WELCOME_USER_TEMPLATE")
                .unwrap_or_else(|| Self::DEFAULT_USER_TEMPLATE.to_owned()),
        }
    }
}

fn read_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.trim().is_empty())
}

fn read_port(name: &str) -> Option<u16> {
    read_env(name).and_then(|value| value.parse::<u16>().ok())
}
