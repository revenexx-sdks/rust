use serde::{Deserialize, Serialize};

/// Collections List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectionList {
    /// List of collections.
    #[serde(rename = "collections", default)]
    pub collections: Vec<crate::models::Collection>,
    /// Total number of collections that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
