use anyhow::Error;
use tokio::sync::watch;

mod config;
mod node;
mod randstat;
mod state;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "badnet=debug")
    }
    tracing_subscriber::fmt()
        .with_env_filter(r#"badnet=debug"#)
        .init();

    let config = config::load_configuration()?;

    tracing::info!(
        "program starts; node count = {}; base port number = {}",
        config.node_count,
        config.base_port_number
    );

    let state_init = vec![randstat::StatInit {
        percentage: 10,
        value: state::Event::Disconnect as u8,
    }];
    let state = state::wrap_shared_state(state::State::new(&state_init)?);

    let mut halt = false;
    let (halt_tx, halt_rx) = watch::channel(halt);

    let mut join_handles = vec![];
    for node_id in 1..=config.node_count {
        let node_state = state.clone();
        let port_number = config.base_port_number + node_id;
        let halt_rx = halt_rx.clone();

        let join_handle = tokio::spawn(async move {
            node::node_runner(node_id, node_state, port_number, halt_rx).await?;

            Ok::<(), Error>(())
        });
        join_handles.push((node_id, join_handle));
    }

    // wait for a signal
    shutdown_signal().await;

    tracing::debug!("sending halt to spawned nodes");
    halt = true;
    halt_tx.send(halt)?;

    for (node_id, join_handle) in join_handles {
        let result = join_handle.await?;
        tracing::debug!("node: {:03}; join result = {:?}", node_id, result);
    }

    Ok(())
}

/// unix signal handler
/// pasted from Axum example
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
