use serde::{Deserialize, Serialize};

/// A Typesense search response, passed through verbatim.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchResult {
    #[serde(rename = "facet_counts", default)]
    pub facet_counts: Vec<crate::models::FacetCount>,
    /// Total matching documents.
    #[serde(rename = "found", default)]
    pub found: i64,
    #[serde(rename = "hits", default)]
    pub hits: Vec<crate::models::SearchHit>,
    /// Documents searched.
    #[serde(rename = "out_of", default)]
    pub out_of: i64,
    /// 1-based page this result is for.
    #[serde(rename = "page", default)]
    pub page: i64,
    #[serde(rename = "search_time_ms", default)]
    pub search_time_ms: i64,
}
