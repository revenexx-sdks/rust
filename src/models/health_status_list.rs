use serde::{Deserialize, Serialize};

/// Status List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthStatusList {
    /// List of statuses.
    #[serde(rename = "statuses", default)]
    pub statuses: Vec<crate::models::HealthStatus>,
    /// Total number of statuses that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
