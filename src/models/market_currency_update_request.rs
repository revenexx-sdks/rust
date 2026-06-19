use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketCurrencyUpdateRequest {
    /// ISO 4217 code, e.g. EUR (unique per market).
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Sort position (default 0).
    #[serde(rename = "position", default)]
    pub position: i64,
}
