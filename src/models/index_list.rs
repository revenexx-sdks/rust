use serde::{Deserialize, Serialize};

/// Indexes List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexList {
    /// List of indexes.
    #[serde(rename = "indexes", default)]
    pub indexes: Vec<crate::models::Index>,
    /// Total number of indexes that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
