use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryAdjustRequest {
    /// The corrections — quantities are SIGNED deltas (at most 200).
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::InventoryAdjustItem>,
    /// Adjusted location (default 'main').
    #[serde(rename = "location_code", default)]
    pub location_code: String,
    /// Mandatory audit reason — every adjustment is a ledger row.
    #[serde(rename = "reason", default)]
    pub reason: String,
}
