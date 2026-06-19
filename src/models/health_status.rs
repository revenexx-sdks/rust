use serde::{Deserialize, Serialize};

/// Health Status
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Name of the service.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Duration in milliseconds how long the health check took.
    #[serde(rename = "ping", default)]
    pub ping: i64,
    /// Service status. Possible values are: `pass`, `fail`
    #[serde(rename = "status", default)]
    pub status: String,
}
