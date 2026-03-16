//! LLM provider clients.
//!
//! Supported providers:
//! - OpenAI
//! - Anthropic
//! - Google Gemini
//! - Azure OpenAI
//! - OpenRouter
//! - Ollama
//! - Groq
//! - Together AI
//!
//! Each provider has its own module with a `Client` implementation for
//! initializing completion and embedding models.
//!
//! # Example
//! ```
//! use rig::{providers::openai, agent::AgentBuilder};
//!
//! let openai = openai::Client::new("your-openai-api-key");
//!
//! let agent = openai.agent("gpt-4o")
//!     .preamble("You are a helpful assistant.")
//!     .build();
//! ```
pub mod anthropic;
pub mod azure;
pub mod gemini;
pub mod groq;
pub mod ollama;
pub mod openai;
pub mod openrouter;
pub mod together;
