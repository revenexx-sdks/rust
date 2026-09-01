use serde::{Deserialize, Serialize};

/// An evenly-stepped tier table. Tiers are generated at from_value,
/// from_value+step, … up to to_value; each costs step_price more than the
/// one before.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingRateTiersLadderRequest {
    /// Price of the first tier.
    #[serde(rename = "base_price", default)]
    pub base_price: f64,
    /// First tier threshold (default 0), in the method's matrix measure.
    #[serde(rename = "from_value", default)]
    pub from_value: f64,
    /// Replace the whole table (default true) or append to it.
    #[serde(rename = "replace", default)]
    pub replace: bool,
    /// Distance between two tiers. Must be > 0.
    #[serde(rename = "step", default)]
    pub step: f64,
    /// Added to each subsequent tier (default 0). A negative value is allowed as
    /// long as no tier ends up below 0.
    #[serde(rename = "step_price", default)]
    pub step_price: f64,
    /// Last tier threshold. The final tier keeps applying above it — a matrix
    /// has no upper bound. Must be >= from_value.
    #[serde(rename = "to_value", default)]
    pub to_value: f64,
}
