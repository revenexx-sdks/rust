use serde::{Deserialize, Serialize};

/// Logs List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogList {
    /// List of logs.
    #[serde(rename = "logs", default)]
    pub logs: Vec<crate::models::Log>,
    /// Total number of logs that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
