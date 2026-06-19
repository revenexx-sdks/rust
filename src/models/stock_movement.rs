use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StockMovement {
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "location_id", default)]
    pub location_id: String,
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    #[serde(rename = "reason", default)]
    pub reason: String,
    #[serde(rename = "sku", default)]
    pub sku: String,
    #[serde(rename = "type", default)]
    pub xtype: String,
}
