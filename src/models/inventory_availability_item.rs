use serde::{Deserialize, Serialize};

/// An item to check: 'product_id' or 'sku'.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryAvailabilityItem {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Requested quantity for the orderable check (default 1).
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    #[serde(rename = "sku", default)]
    pub sku: String,
}
