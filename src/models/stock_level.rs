use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StockLevel {
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "location_id", default)]
    pub location_id: String,
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "on_hand", default)]
    pub on_hand: f64,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "reorder_point", default)]
    pub reorder_point: f64,
    #[serde(rename = "reserved", default)]
    pub reserved: f64,
    #[serde(rename = "sku", default)]
    pub sku: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
