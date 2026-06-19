use serde::{Deserialize, Serialize};

/// An item and its SIGNED correction: 'product_id' or 'sku'.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryAdjustItem {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Signed delta (±on_hand) — must be non-zero.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    #[serde(rename = "sku", default)]
    pub sku: String,
}
