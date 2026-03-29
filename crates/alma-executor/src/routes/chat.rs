use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::service::{AlmaMessage, AlmaRole};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ChatHistoryEntry {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
    #[serde(default)]
    pub history: Vec<ChatHistoryEntry>,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub response: String,
}

pub async fn chat(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, String)> {
    let history: Vec<AlmaMessage> = req
        .history
        .into_iter()
        .map(|entry| AlmaMessage {
            role: match entry.role.as_str() {
                "assistant" => AlmaRole::Assistant,
                _ => AlmaRole::User,
            },
            content: entry.content,
        })
        .collect();

    let response = state
        .agent
        .chat(&req.message, history)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(ChatResponse { response }))
}
