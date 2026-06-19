use serde::{Deserialize, Serialize};

/// A new matrix tier (from_value → price) of the method in the path.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingRateTierCreateRequest {
    /// Tier threshold (default 0) — the tier with the highest from_value at or
    /// below the measured value wins.
    #[serde(rename = "from_value", default)]
    pub from_value: f64,
    /// Sort order (default 0; bulk replace derives it from the array index).
    #[serde(rename = "position", default)]
    pub position: i64,
    /// Price of this tier (default 0).
    #[serde(rename = "price", default)]
    pub price: f64,
}
