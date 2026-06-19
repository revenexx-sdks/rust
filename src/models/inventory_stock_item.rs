use serde::{Deserialize, Serialize};

/// An item and its quantity: 'product_id' or 'sku'.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryStockItem {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    #[serde(rename = "sku", default)]
    pub sku: String,
}
