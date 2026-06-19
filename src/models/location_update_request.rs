use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocationUpdateRequest {
    #[serde(rename = "address", default)]
    pub address: serde_json::Value,
    /// Unique location code (per tenant).
    #[serde(rename = "code", default)]
    pub code: String,
    /// Disabled locations are skipped by availability and reserve (default true).
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// Localised display names ({de, en, …}).
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sourcing order — lower wins (default 0).
    #[serde(rename = "priority", default)]
    pub priority: i64,
    /// Default 'warehouse'.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
