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

use alma_memory::AlmaMemory;

use crate::{adapter::OpenRouterAdapter, service::AlmaAgent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cfg = config::Config::from_env();
    let port = cfg.port;

    let adapter = OpenRouterAdapter::from_config(&cfg)?;
    let agent = Arc::new(AlmaAgent::new(adapter));
    let memory = AlmaMemory::new(&cfg.qdrant_url, &cfg.memory_collection);
    let state = Arc::new(state::AppState::new(agent, memory, cfg));

    let app = Router::new()
        .route("/health", get(routes::health::health))
        .route("/v1/prompt", post(routes::prompt::prompt))
        .route("/v1/chat", post(routes::chat::chat))
        .route("/v1/stream", post(routes::stream::stream))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("alma-executor listening on {addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
