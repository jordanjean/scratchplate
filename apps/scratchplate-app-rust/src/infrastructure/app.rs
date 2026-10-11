use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::sync::watch;
use tracing::info;

use crate::adapter::inbound::web::{build_main_router, build_management_router, WebState};
use crate::adapter::outbound::config::ConfigWelcomeTemplateAdapter;
use crate::application::service::GreetUserService;
use crate::infrastructure::config::AppConfig;
use crate::infrastructure::security::{JwtVerifier, JwksKeySource};

pub async fn run(config: AppConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
    let http_client = reqwest::Client::builder().build()?;
    let welcome_templates = Arc::new(ConfigWelcomeTemplateAdapter::new(
        config.welcome_admin_template.clone(),
        config.welcome_user_template.clone(),
    ));
    let greet_user = Arc::new(GreetUserService::new(welcome_templates));
    let key_source = Arc::new(JwksKeySource::new(
        http_client,
        config.jwt_jwk_set_uri.clone(),
        Duration::from_secs(30),
    ));
    let verifier = Arc::new(JwtVerifier::new(config.jwt_issuer_uri.clone(), key_source));

    let combined_port = config.server_port == config.management_server_port;
    let main_state = Arc::new(WebState::new(greet_user.clone(), verifier, combined_port));
    let main_router = build_main_router(main_state, combined_port);

    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let shutdown_task = tokio::spawn(async move {
        shutdown_signal().await;
        let _ = shutdown_tx.send(true);
    });

    if combined_port {
        let address = socket_addr(config.server_port);
        info!(%address, "starting application and management listener");
        let listener = TcpListener::bind(address).await?;
        axum::serve(listener, main_router)
            .with_graceful_shutdown(wait_for_shutdown(shutdown_rx.clone()))
            .await?;
    } else {
        let management_router = build_management_router();
        let app_address = socket_addr(config.server_port);
        let management_address = socket_addr(config.management_server_port);

        info!(%app_address, %management_address, "starting dedicated application and management listeners");

        let app_listener = TcpListener::bind(app_address).await?;
        let management_listener = TcpListener::bind(management_address).await?;

        let app_server = axum::serve(app_listener, main_router)
            .with_graceful_shutdown(wait_for_shutdown(shutdown_rx.clone()));
        let management_server = axum::serve(management_listener, management_router)
            .with_graceful_shutdown(wait_for_shutdown(shutdown_rx.clone()));

        tokio::try_join!(app_server, management_server)?;
    }

    shutdown_task.abort();
    Ok(())
}

fn socket_addr(port: u16) -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], port))
}

async fn wait_for_shutdown(mut receiver: watch::Receiver<bool>) {
    if *receiver.borrow() {
        return;
    }

    let _ = receiver.changed().await;
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        let mut terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to register SIGTERM handler");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = terminate.recv() => {},
        }
    }

    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }

    info!("shutdown signal received");
}
