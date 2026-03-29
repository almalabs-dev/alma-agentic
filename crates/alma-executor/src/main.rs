mod adapter;
mod config;
mod routes;
mod service;
mod state;

use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cfg = config::Config::from_env();
    let port = cfg.port;

    let state = Arc::new(state::AppState::new(cfg));

    let app = Router::new()
        .route("/health", get(routes::health::health))
        .route("/v1/prompt", post(routes::prompt::prompt))
        .route("/v1/chat", post(routes::chat::chat))
        .route("/v1/stream", post(routes::stream::stream))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("alma-executor listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
