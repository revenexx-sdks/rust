use serde::{Deserialize, Serialize};

/// Sessions List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionList {
    /// List of sessions.
    #[serde(rename = "sessions", default)]
    pub sessions: Vec<crate::models::Session>,
    /// Total number of sessions that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
