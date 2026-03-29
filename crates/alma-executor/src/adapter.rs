/// OpenRouterAdapter — the only file in alma-executor that imports rig::*.
///
/// All provider types, traits, and streaming details are confined here.
/// The rest of the crate depends only on crate::service::{AlmaMessage, AlmaRole,
/// AgentEvent, AgentError} and crate::service::AlmaAgent.
use std::pin::Pin;

use async_stream::stream;
use futures::{Stream, StreamExt};
use rig::{
    agent::MultiTurnStreamItem,
    client::{CompletionClient, ProviderClient},
    completion::{Chat, Message, Prompt},
    providers::openrouter,
    streaming::{StreamedAssistantContent, StreamingPrompt},
};

use crate::config::Config;
use crate::service::{AgentError, AgentEvent, AlmaMessage, AlmaRole};

// ---------------------------------------------------------------------------
// Adapter
// ---------------------------------------------------------------------------

pub struct OpenRouterAdapter {
    agent: rig::agent::Agent<openrouter::CompletionModel>,
}

impl OpenRouterAdapter {
    pub fn from_config(cfg: &Config) -> Self {
        let client = openrouter::Client::from_env();
        let agent = client
            .agent(&cfg.default_model)
            .preamble(&cfg.preamble)
            .build();
        Self { agent }
    }

    pub async fn prompt(&self, message: &str) -> Result<String, AgentError> {
        self.agent
            .prompt(message)
            .await
            .map_err(|e| AgentError::new(e.to_string()))
    }

    pub async fn chat(
        &self,
        message: &str,
        history: Vec<AlmaMessage>,
    ) -> Result<String, AgentError> {
        let rig_history: Vec<Message> = history
            .into_iter()
            .map(|m| match m.role {
                AlmaRole::Assistant => Message::assistant(m.content),
                AlmaRole::User => Message::user(m.content),
            })
            .collect();

        self.agent
            .chat(message, rig_history)
            .await
            .map_err(|e| AgentError::new(e.to_string()))
    }

    pub fn stream(&self, message: String) -> Pin<Box<dyn Stream<Item = AgentEvent> + Send>> {
        let agent = self.agent.clone();

        Box::pin(stream! {
            let mut raw = agent.stream_prompt(&message).await;

            while let Some(item) = raw.next().await {
                match item {
                    Ok(MultiTurnStreamItem::StreamAssistantItem(
                        StreamedAssistantContent::Text(text),
                    )) => yield AgentEvent::Text(text.text),

                    Ok(MultiTurnStreamItem::FinalResponse(_)) => {
                        yield AgentEvent::Done;
                        return;
                    }

                    Ok(_) => {}

                    Err(e) => {
                        yield AgentEvent::Error(AgentError::new(e.to_string()));
                        return;
                    }
                }
            }
        })
    }
}
