pub mod adapter;
pub mod application;
pub mod domain;
pub mod infrastructure;

pub use adapter::inbound::web::{build_main_router, build_management_router, WebState};
pub use infrastructure::app::run;
pub use infrastructure::config::AppConfig;
pub use infrastructure::security::{AuthenticationError, JwksKeySource, JwtClaims, JwtVerifier, KeySource, RealmAccessClaims, StaticKeySource};
