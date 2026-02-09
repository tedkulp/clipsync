mod handler;
mod room;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::room::RoomManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "clipsync_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get configuration from environment
    let port = std::env::var("CLIPSYNC_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let max_history = std::env::var("CLIPSYNC_MAX_HISTORY")
        .ok()
        .and_then(|h| h.parse().ok())
        .unwrap_or(50);

    tracing::info!("Starting ClipSync server on port {}", port);
    tracing::info!("Max history per room: {}", max_history);

    // Create room manager
    let room_manager = Arc::new(RoomManager::new(max_history));

    // Build router
    let app = Router::new()
        .route("/ws", get(handler::websocket_handler))
        .route("/health", get(|| async { "OK" }))
        .with_state(room_manager)
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
