use serde::{Deserialize, Serialize};

/// A matrix tier of the new set (from_value → price) — null falls back to
/// 0, position derives from the array order.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingRateTierReplaceItem {
    /// Tier threshold (default 0) — the tier with the highest from_value at or
    /// below the measured value wins.
    #[serde(rename = "from_value", default)]
    pub from_value: f64,
    /// Ignored — derived from the array index.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// Price of this tier (default 0).
    #[serde(rename = "price", default)]
    pub price: f64,
}
