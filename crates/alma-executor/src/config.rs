use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub default_model: String,
    pub preamble: String,
    pub qdrant_url: String,
    pub memory_collection: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            port: env::var("ALMA_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3000),
            default_model: env::var("ALMA_DEFAULT_MODEL")
                .unwrap_or_else(|_| "anthropic/claude-sonnet-4".to_string()),
            preamble: env::var("ALMA_PREAMBLE").unwrap_or_else(|_| {
                "You are a helpful AI assistant powered by Alma Labs.".to_string()
            }),
            qdrant_url: env::var("QDRANT_URL")
                .unwrap_or_else(|_| "http://localhost:6334".to_string()),
            memory_collection: env::var("ALMA_MEMORY_COLLECTION")
                .unwrap_or_else(|_| "alma".to_string()),
        }
    }
}
