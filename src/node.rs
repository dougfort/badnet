use anyhow::Error;
use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};
use tokio::sync::watch;
use tower::ServiceBuilder;
use tower_http::{add_extension::AddExtensionLayer, trace::TraceLayer};

use crate::state;

pub async fn node_runner(
    node_id: usize,
    state: state::SharedState,
    port_number: usize,
    mut halt_rx: watch::Receiver<bool>,
) -> Result<(), Error> {
    // build our application
    let app = Router::new()
        .route("/", get(index))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(state));

    // run it with hyper
    let addr = format!("0.0.0.0:{}", port_number).parse()?;
    tracing::debug!("node: {:03}; listening on {}", node_id, addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async { halt_rx.changed().await.unwrap() })
        .await?;

    tracing::info!("node: {:03}; exits", node_id);
    Ok(())
}

/// HTTP handler for GET /
async fn index() -> String {
    "Hello, World!\n".to_string()
}
