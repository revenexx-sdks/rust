use serde::{Deserialize, Serialize};

/// Teams List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TeamList {
    /// List of teams.
    #[serde(rename = "teams", default)]
    pub teams: Vec<crate::models::Team>,
    /// Total number of teams that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
