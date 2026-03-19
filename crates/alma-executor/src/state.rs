use rig::{
    agent::Agent,
    client::{CompletionClient, ProviderClient},
    providers::openrouter,
};

use crate::config::Config;

pub type OpenRouterAgent = Agent<openrouter::CompletionModel>;

pub struct AppState {
    pub agent: OpenRouterAgent,
    #[allow(dead_code)] // config used in Phase 2b for per-request model selection
    pub config: Config,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let client = openrouter::Client::from_env();
        let agent = client
            .agent(&config.default_model)
            .preamble(&config.preamble)
            .build();
        Self { agent, config }
    }
}
