use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelCreateRequest {
    /// Stable channel code, unique per tenant (e.g. shop, punchout-acme).
    #[serde(rename = "code", default)]
    pub code: String,
    /// Mark as the default channel (default false).
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Localized display names keyed by locale.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Display name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sort position (default 0).
    #[serde(rename = "position", default)]
    pub position: i64,
    /// Lifecycle status (default 'active').
    #[serde(rename = "status", default)]
    pub status: String,
    /// Where business happens (default 'storefront').
    #[serde(rename = "type", default)]
    pub xtype: String,
}
