use serde::{Deserialize, Serialize};

/// The owning market comes from the route path ('market_id').
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketTaxClassCreateRequest {
    /// Tax class code (unique per market).
    #[serde(rename = "code", default)]
    pub code: String,
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
    /// Tax rate in percent, 0–100 (default 0).
    #[serde(rename = "rate", default)]
    pub rate: f64,
}
