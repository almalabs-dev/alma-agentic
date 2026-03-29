use std::pin::Pin;

use futures::Stream;

use crate::adapter::OpenRouterAdapter;

// ---------------------------------------------------------------------------
// Alma-typed message model
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlmaMessage {
    pub role: AlmaRole,
    pub content: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlmaRole {
    User,
    Assistant,
}

// ---------------------------------------------------------------------------
// Streaming event type
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum AgentEvent {
    Text(String),
    Done,
    Error(AgentError),
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentError(String);

impl AgentError {
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl std::fmt::Display for AgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for AgentError {}

// ---------------------------------------------------------------------------
// AlmaAgent — the Alma-facing agent.
// Routes depend only on this struct; they never see provider types.
// All provider internals live in adapter.rs.
// ---------------------------------------------------------------------------

pub struct AlmaAgent {
    inner: OpenRouterAdapter,
}

impl AlmaAgent {
    pub fn new(inner: OpenRouterAdapter) -> Self {
        Self { inner }
    }

    pub async fn prompt(&self, message: &str) -> Result<String, AgentError> {
        self.inner.prompt(message).await
    }

    pub async fn chat(
        &self,
        message: &str,
        history: Vec<AlmaMessage>,
    ) -> Result<String, AgentError> {
        self.inner.chat(message, history).await
    }

    pub fn stream(&self, message: String) -> Pin<Box<dyn Stream<Item = AgentEvent> + Send>> {
        self.inner.stream(message)
    }
}
