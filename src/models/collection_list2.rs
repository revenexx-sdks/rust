use serde::{Deserialize, Serialize};

/// Collections List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectionList2 {
    /// List of collections.
    #[serde(rename = "collections", default)]
    pub collections: Vec<crate::models::Collection2>,
    /// Total number of collections that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
