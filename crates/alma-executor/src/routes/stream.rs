use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::sse::{Event, Sse},
};
use futures::StreamExt;
use rig::{
    agent::{MultiTurnStreamItem, StreamingError},
    providers::openrouter::streaming::StreamingCompletionResponse,
    streaming::{StreamedAssistantContent, StreamingPrompt},
};
use serde::Deserialize;
use serde_json::json;
use std::{convert::Infallible, sync::Arc};

use crate::state::AppState;

#[derive(Deserialize)]
pub struct StreamRequest {
    pub message: String,
}

type StreamItem = Result<MultiTurnStreamItem<StreamingCompletionResponse>, StreamingError>;

pub async fn stream(
    State(state): State<Arc<AppState>>,
    Json(req): Json<StreamRequest>,
) -> Result<Sse<impl futures::Stream<Item = Result<Event, Infallible>>>, (StatusCode, String)> {
    // stream_prompt().await returns the stream directly (not a Result)
    let raw_stream = state
        .agent
        .stream_prompt(req.message.as_str())
        .await;

    let sse_stream = raw_stream.filter_map(|item: StreamItem| async move {
        match item {
            Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(
                text,
            ))) => {
                let data = json!({ "text": text.text }).to_string();
                Some(Ok(Event::default().data(data)))
            }
            Ok(MultiTurnStreamItem::FinalResponse(_)) => {
                Some(Ok(Event::default().data("[DONE]")))
            }
            Ok(_) => None,
            Err(e) => {
                let data = json!({ "error": e.to_string() }).to_string();
                Some(Ok(Event::default().event("error").data(data)))
            }
        }
    });

    Ok(Sse::new(sse_stream))
}
