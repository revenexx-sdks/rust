use serde::{Deserialize, Serialize};

/// A stock row tracks an item: 'product_id' or 'sku'.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StockLevelCreateRequest {
    /// Owning location.
    #[serde(rename = "location_id", default)]
    pub location_id: String,
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Physical stock (default 0).
    #[serde(rename = "on_hand", default)]
    pub on_hand: f64,
    /// Tracked product.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "reorder_point", default)]
    pub reorder_point: f64,
    /// Reserved stock (default 0) — normally managed by reserve/release/commit.
    #[serde(rename = "reserved", default)]
    pub reserved: f64,
    /// Tracked SKU (alternative to product_id).
    #[serde(rename = "sku", default)]
    pub sku: String,
}
