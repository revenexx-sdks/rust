use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingRateTiersReplaceRequest {
    /// The complete new tier set (set semantics) — positions are derived from
    /// the array order.
    #[serde(rename = "tiers", default)]
    pub tiers: Vec<crate::models::ShippingRateTierReplaceItem>,
}
