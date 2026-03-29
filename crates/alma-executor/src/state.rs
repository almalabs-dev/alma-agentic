use crate::config::Config;
use crate::service::AlmaAgent;
use std::sync::Arc;

pub struct AppState {
    pub agent: Arc<AlmaAgent>,
    #[allow(dead_code)] // config used in later phases
    pub config: Config,
}

impl AppState {
    pub fn new(agent: Arc<AlmaAgent>, config: Config) -> Self {
        Self { agent, config }
    }
}
