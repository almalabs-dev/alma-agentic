use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::sse::{Event, Sse},
};
use futures::StreamExt;
use serde::Deserialize;
use serde_json::json;
use std::{convert::Infallible, sync::Arc};

use crate::service::AgentEvent;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct StreamRequest {
    pub message: String,
}

pub async fn stream(
    State(state): State<Arc<AppState>>,
    Json(req): Json<StreamRequest>,
) -> Result<Sse<impl futures::Stream<Item = Result<Event, Infallible>>>, (StatusCode, String)> {
    let event_stream = state.agent.stream(req.message);

    let sse_stream = event_stream.map(|event| {
        Ok(match event {
            AgentEvent::Text(text) => {
                Event::default().data(json!({ "text": text }).to_string())
            }
            AgentEvent::Done => Event::default().data("[DONE]"),
            AgentEvent::Error(e) => Event::default()
                .event("error")
                .data(json!({ "error": e }).to_string()),
        })
    });

    Ok(Sse::new(sse_stream))
}
