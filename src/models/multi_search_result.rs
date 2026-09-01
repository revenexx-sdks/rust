use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MultiSearchResult {
    /// One result per entry in `searches`, in the same order.
    #[serde(rename = "results", default)]
    pub results: Vec<crate::models::SearchResult>,
}
