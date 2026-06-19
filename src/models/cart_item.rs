use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartItem {
    #[serde(rename = "cart_id", default)]
    pub cart_id: String,
    #[serde(rename = "configuration", default)]
    pub configuration: serde_json::Value,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "currency", default)]
    pub currency: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "line_total", default)]
    pub line_total: f64,
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    #[serde(rename = "sku", default)]
    pub sku: String,
    #[serde(rename = "snapshot", default)]
    pub snapshot: serde_json::Value,
    #[serde(rename = "tax_rate", default)]
    pub tax_rate: f64,
    #[serde(rename = "type", default)]
    pub xtype: String,
    #[serde(rename = "unit", default)]
    pub unit: String,
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
