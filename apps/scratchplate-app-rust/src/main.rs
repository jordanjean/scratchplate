use scratchplate_app_rust::{run, AppConfig};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();

    let config = AppConfig::from_env();

    if let Err(error) = run(config).await {
        tracing::error!(error = %error, "application exited with error");
        std::process::exit(1);
    }
}
