use serde::{Deserialize, Serialize};

/// Documents List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentList {
    /// List of documents.
    #[serde(rename = "documents", default)]
    pub documents: Vec<crate::models::Document>,
    /// Total number of documents that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
