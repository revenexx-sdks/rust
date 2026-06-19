use serde::{Deserialize, Serialize};

/// Runtimes List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuntimeList {
    /// List of runtimes.
    #[serde(rename = "runtimes", default)]
    pub runtimes: Vec<crate::models::Runtime>,
    /// Total number of runtimes that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
