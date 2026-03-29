use std::sync::Arc;

use alma_memory::AlmaMemory;

use crate::config::Config;
use crate::service::AlmaAgent;

pub struct AppState {
    pub agent: Arc<AlmaAgent>,
    #[allow(dead_code)] // wired; used when conversation-persistence front opens
    pub memory: AlmaMemory,
    #[allow(dead_code)] // config retained for later phases
    pub config: Config,
}

impl AppState {
    pub fn new(agent: Arc<AlmaAgent>, memory: AlmaMemory, config: Config) -> Self {
        Self {
            agent,
            memory,
            config,
        }
    }
}
