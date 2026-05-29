use srvcs_www::{config::Config, health, router, service_name, telemetry};

async fn shutdown_signal() {
    let ctrl_c = async { tokio::signal::ctrl_c().await.expect("ctrl_c") };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("SIGTERM")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! { _ = ctrl_c => {}, _ = terminate => {} }
    health::set_ready(false);
    tracing::info!("shutdown signal received; draining");
}

#[tokio::main]
async fn main() {
    let cfg = Config::from_env();
    telemetry::init(&cfg.log_level);
    let metrics = telemetry::install_metrics();
    health::set_ready(true);
    let listener = tokio::net::TcpListener::bind(cfg.bind_addr).await.unwrap();
    tracing::info!(addr = %cfg.bind_addr, env = %cfg.environment, service = service_name(), "listening");
    axum::serve(listener, router(metrics))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
