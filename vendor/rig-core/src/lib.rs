#![cfg_attr(docsrs, feature(doc_cfg))]
//! Core library for alma-agentic — the orchestration layer of Alma Labs.
//!
//! Provides multi-provider LLM support, agent orchestration, streaming,
//! embeddings, tool dispatch, and vector store integration.
//!
//! # High-level features
//! - Full support for LLM completion and embedding workflows
//! - Multi-provider support: OpenAI, Anthropic, Gemini, Azure, OpenRouter, Ollama, Groq, Together
//! - Agent orchestration with tool calling and streaming
//! - Integrate LLMs in your app with minimal boilerplate
//!
//! # Simple example:
//! ```
//! use rig::{client::CompletionClient, completion::Prompt, providers::openai};
//!
//! #[tokio::main]
//! async fn main() {
//!     let openai_client = openai::Client::from_env();
//!     let agent = openai_client.agent("gpt-4o").build();
//!
//!     let response = agent
//!         .prompt("Who are you?")
//!         .await
//!         .expect("Failed to prompt");
//!
//!     println!("{response}");
//! }
//! ```
//!
//! # Core concepts
//! ## Completion and embedding models
//! Each provider has a `Client` struct that can be used to initialize completion
//! and embedding models. These models implement the [CompletionModel](crate::completion::CompletionModel)
//! and [EmbeddingModel](crate::embeddings::EmbeddingModel) traits respectively.
//!
//! ## Agents
//! The [Agent](crate::agent::Agent) type provides high-level abstractions over LLMs,
//! from simple agents to full RAG systems with knowledge bases.
//!
//! ## Vector stores and indexes
//! The [VectorStoreIndex](crate::vector_store::VectorStoreIndex) trait defines a common
//! interface for vector stores. Qdrant integration is available via the companion crate.
//!
//! # Supported Providers
//! - Anthropic, Azure, Gemini, Groq, Ollama, OpenAI, OpenRouter, Together
//!
//! Custom providers can be added by implementing the
//! [CompletionModel](crate::completion::CompletionModel) and
//! [EmbeddingModel](crate::embeddings::EmbeddingModel) traits.
//!

extern crate self as rig;

pub mod agent;
#[cfg(feature = "audio")]
#[cfg_attr(docsrs, doc(cfg(feature = "audio")))]
pub mod audio_generation;
pub mod client;
pub mod completion;
pub mod embeddings;

#[cfg(feature = "experimental")]
#[cfg_attr(docsrs, doc(cfg(feature = "experimental")))]
pub mod evals;
pub mod extractor;
pub mod http_client;
#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
pub mod image_generation;
pub mod integrations;
pub(crate) mod json_utils;
pub mod loaders;
pub mod model;
pub mod one_or_many;
pub mod pipeline;
pub mod prelude;
pub mod providers;

pub mod streaming;
pub mod tool;
pub mod tools;
pub mod transcription;
pub mod vector_store;
pub mod wasm_compat;

// Re-export commonly used types and traits
pub use completion::message;
pub use embeddings::Embed;
pub use extractor::ExtractionResponse;
pub use one_or_many::{EmptyListError, OneOrMany};

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use rig_derive::{Embed, rig_tool as tool_macro};

pub mod telemetry;
