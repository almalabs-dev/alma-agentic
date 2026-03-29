//! alma-memory — Alma Labs memory boundary.
//!
//! Public API is Alma-owned. All Qdrant types are confined to `adapter`.
//! Consumers of this crate never import `qdrant_client::*`.

mod adapter;
mod types;

pub use types::{AlmaMemory, MemoryEntry, MemoryError, MemoryHit, MemoryQuery};
