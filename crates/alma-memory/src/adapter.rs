/// QdrantMemoryAdapter — the only file in alma-memory that imports qdrant_client::*.
///
/// All Qdrant types, traits, and protocol details are confined here.
/// The rest of the crate depends only on the Alma types exported from types.rs.
use qdrant_client::{
    Payload, Qdrant,
    qdrant::{
        GetPointsBuilder, PointId, PointStruct, UpsertPointsBuilder,
        point_id::PointIdOptions,
    },
};

use crate::types::{MemoryEntry, MemoryError, MemoryHit, MemoryQuery};

// ---------------------------------------------------------------------------
// Adapter
// ---------------------------------------------------------------------------

pub(crate) struct QdrantMemoryAdapter {
    url: String,
    collection: String,
}

impl QdrantMemoryAdapter {
    pub(crate) fn new(url: impl Into<String>, collection: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            collection: collection.into(),
        }
    }

    fn client(&self) -> Result<Qdrant, MemoryError> {
        Qdrant::from_url(&self.url)
            .build()
            .map_err(|e| MemoryError::backend(e.to_string()))
    }

    pub(crate) async fn store(&self, entry: MemoryEntry) -> Result<(), MemoryError> {
        let client = self.client()?;

        // Payload carries the Alma content and metadata.
        // A zero vector is used as a placeholder until an embedding model is
        // integrated in the conversation-persistence front.
        let payload = Payload::try_from(
            serde_json::json!({
                "content": entry.content,
                "metadata": entry.metadata,
            }),
        )
        .map_err(|e| MemoryError::backend(e.to_string()))?;

        let point = PointStruct::new(entry.id, vec![0.0_f32], payload);

        client
            .upsert_points(
                UpsertPointsBuilder::new(&self.collection, vec![point]).wait(true),
            )
            .await
            .map_err(|e| MemoryError::backend(e.to_string()))?;

        Ok(())
    }

    pub(crate) async fn search(
        &self,
        _query: MemoryQuery,
    ) -> Result<Vec<MemoryHit>, MemoryError> {
        // Semantic search requires a pre-computed query vector from an
        // embedding model. That integration is deferred to the
        // conversation-persistence front.
        Err(MemoryError::not_connected(
            "search requires an embedding model — deferred to conversation-persistence front",
        ))
    }

    pub(crate) async fn recall(&self, id: &str) -> Result<Option<MemoryEntry>, MemoryError> {
        let client = self.client()?;

        let results = client
            .get_points(
                GetPointsBuilder::new(
                    &self.collection,
                    vec![PointId {
                        point_id_options: Some(PointIdOptions::Uuid(id.to_string())),
                    }],
                )
                .with_payload(true),
            )
            .await
            .map_err(|e| MemoryError::backend(e.to_string()))?;

        let point = match results.result.into_iter().next() {
            Some(p) => p,
            None => return Ok(None),
        };

        // Deserialize payload through serde_json.
        let value = serde_json::to_value(point.payload)
            .map_err(|e| MemoryError::backend(e.to_string()))?;

        let content = value
            .get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let metadata = value
            .get("metadata")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        Ok(Some(MemoryEntry {
            id: id.to_string(),
            content,
            metadata,
        }))
    }
}
