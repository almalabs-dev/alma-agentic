use axum::{
    Json,
    extract::State,
    http::StatusCode,
};
use rig::completion::Prompt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct PromptRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct PromptResponse {
    pub response: String,
}

pub async fn prompt(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PromptRequest>,
) -> Result<Json<PromptResponse>, (StatusCode, String)> {
    let response = state
        .agent
        .prompt(req.message.as_str())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(PromptResponse { response }))
}
