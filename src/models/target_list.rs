use serde::{Deserialize, Serialize};

/// Target list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetList {
    /// List of targets.
    #[serde(rename = "targets", default)]
    pub targets: Vec<crate::models::Target>,
    /// Total number of targets that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
