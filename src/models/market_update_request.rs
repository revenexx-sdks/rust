use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketUpdateRequest {
    /// Market code (unique per tenant).
    #[serde(rename = "code", default)]
    pub code: String,
    /// ISO 4217 code (default 'EUR').
    #[serde(rename = "currency", default)]
    pub currency: String,
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Localized display names ({locale: label}).
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sort position (default 0).
    #[serde(rename = "position", default)]
    pub position: i64,
    /// Default 'active'.
    #[serde(rename = "status", default)]
    pub status: String,
}
