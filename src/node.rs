use anyhow::Error;
use tokio::sync::watch;

pub async fn node_runner(
    node_id: usize,
    _port_number: usize,
    mut halt_rx: watch::Receiver<bool>,
) -> Result<(), Error> {
    let mut halt = *halt_rx.borrow();

    tracing::info!("node: {:03}; starts", node_id);
    while !halt {
        tokio::select! {
            _ = halt_rx.changed() => {halt = *halt_rx.borrow()}
        }
    }

    tracing::info!("node: {:03}; exits", node_id);
    Ok(())
}
