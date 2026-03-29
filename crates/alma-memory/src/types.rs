use std::collections::HashMap;

use crate::adapter::QdrantMemoryAdapter;

// ---------------------------------------------------------------------------
// Alma-typed data model
// ---------------------------------------------------------------------------

/// An entry to be stored in memory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryEntry {
    /// Stable identifier for the entry (UUID or content-addressed string).
    pub id: String,
    /// Raw text content.
    pub content: String,
    /// Arbitrary key-value metadata (agent name, timestamp, kind, etc.).
    pub metadata: HashMap<String, String>,
}

/// A query to retrieve entries from memory.
#[derive(Clone, Debug)]
pub struct MemoryQuery {
    /// Natural-language search text. Requires an embedding model to execute.
    pub text: String,
    /// Maximum number of hits to return.
    pub limit: usize,
}

/// A single result from a memory search.
#[derive(Clone, Debug)]
pub struct MemoryHit {
    pub id: String,
    pub content: String,
    /// Similarity score in [0.0, 1.0]; higher is closer.
    pub score: f32,
    pub metadata: HashMap<String, String>,
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// An error originating from an alma-memory operation.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum MemoryError {
    #[error("backend error: {0}")]
    Backend(String),
    #[error("not connected: {0}")]
    NotConnected(String),
}

impl MemoryError {
    pub fn backend(msg: impl Into<String>) -> Self {
        Self::Backend(msg.into())
    }
    pub fn not_connected(msg: impl Into<String>) -> Self {
        Self::NotConnected(msg.into())
    }
}

// ---------------------------------------------------------------------------
// AlmaMemory — the Alma-facing memory boundary.
// Callers depend only on this struct and the types above; they never see
// qdrant_client::* or any other vendor type.
// ---------------------------------------------------------------------------

pub struct AlmaMemory {
    inner: QdrantMemoryAdapter,
}

impl AlmaMemory {
    /// Build an `AlmaMemory` connected to the given Qdrant URL and collection.
    ///
    /// Does not make a network connection at construction time. The first
    /// actual operation (store / recall) creates the gRPC channel.
    pub fn new(qdrant_url: impl Into<String>, collection: impl Into<String>) -> Self {
        Self {
            inner: QdrantMemoryAdapter::new(qdrant_url, collection),
        }
    }

    /// Store a memory entry. Uses a zero vector as placeholder until an
    /// embedding model is wired in the conversation-persistence front.
    pub async fn store(&self, entry: MemoryEntry) -> Result<(), MemoryError> {
        self.inner.store(entry).await
    }

    /// Semantic search over stored entries. Requires an embedding model;
    /// returns [`MemoryError::NotConnected`] until that integration is open.
    pub async fn search(&self, query: MemoryQuery) -> Result<Vec<MemoryHit>, MemoryError> {
        self.inner.search(query).await
    }

    /// Retrieve a single entry by its identifier.
    pub async fn recall(&self, id: &str) -> Result<Option<MemoryEntry>, MemoryError> {
        self.inner.recall(id).await
    }
}
