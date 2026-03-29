use std::pin::Pin;

use futures::Stream;

use crate::adapter::RigOpenRouterAdapter;

// ---------------------------------------------------------------------------
// Alma-typed message model
// ---------------------------------------------------------------------------

pub struct AlmaMessage {
    pub role: AlmaRole,
    pub content: String,
}

pub enum AlmaRole {
    User,
    Assistant,
}

// ---------------------------------------------------------------------------
// Streaming event type
// ---------------------------------------------------------------------------

pub enum AgentEvent {
    Text(String),
    Done,
    Error(String),
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct AgentError(pub String);

impl std::fmt::Display for AgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

// ---------------------------------------------------------------------------
// AlmaAgent — the Alma-facing agent.
// Routes depend only on this struct; they never see rig::*.
// All Rig internals live in adapter.rs.
// ---------------------------------------------------------------------------

pub struct AlmaAgent {
    inner: RigOpenRouterAdapter,
}

impl AlmaAgent {
    pub fn new(inner: RigOpenRouterAdapter) -> Self {
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
