use crate::adapter::RigOpenRouterAdapter;
use crate::config::Config;
use crate::service::AlmaAgent;

pub struct AppState {
    pub agent: AlmaAgent,
}

impl AppState {
    pub fn new(cfg: Config) -> Self {
        let adapter = RigOpenRouterAdapter::from_config(&cfg);
        Self {
            agent: AlmaAgent::new(adapter),
        }
    }
}
