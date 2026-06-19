use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderItem {
    #[serde(rename = "configuration", default)]
    pub configuration: serde_json::Value,
    #[serde(rename = "cost_center", default)]
    pub cost_center: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "line_total", default)]
    pub line_total: f64,
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "position_text", default)]
    pub position_text: String,
    #[serde(rename = "product", default)]
    pub product: serde_json::Value,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    #[serde(rename = "quantity_cancelled", default)]
    pub quantity_cancelled: f64,
    #[serde(rename = "quantity_returned", default)]
    pub quantity_returned: f64,
    #[serde(rename = "quantity_shipped", default)]
    pub quantity_shipped: f64,
    #[serde(rename = "sku", default)]
    pub sku: String,
    #[serde(rename = "tax_amount", default)]
    pub tax_amount: f64,
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
    #[serde(rename = "user_data", default)]
    pub user_data: serde_json::Value,
}
