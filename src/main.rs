use anyhow::Error;

mod config;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "badnet=debug")
    }
    tracing_subscriber::fmt::init();

    let config = config::load_configuration()?;

    tracing::info!("program starts; node count = {}; base port number = {}", config.node_count, config.base_port_number);

    shutdown_signal().await;
    Ok(())
}

/// unix signal handler
/// pasted from Axum exampe
async fn shutdown_signal() {
    use std::io;
    use tokio::signal::unix::SignalKind;

    async fn terminate() -> io::Result<()> {
        tokio::signal::unix::signal(SignalKind::terminate())?
            .recv()
            .await;
        Ok(())
    }

    tokio::select! {
        _ = terminate() => {},
        _ = tokio::signal::ctrl_c() => {},
    }
    tracing::info!("signal received, starting graceful shutdown")
}
