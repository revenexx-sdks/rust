use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceEntry {
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "price_list_id", default)]
    pub price_list_id: String,
    #[serde(rename = "price_type", default)]
    pub price_type: String,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "quantity_min", default)]
    pub quantity_min: f64,
    #[serde(rename = "sku", default)]
    pub sku: String,
    #[serde(rename = "unit", default)]
    pub unit: String,
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    #[serde(rename = "valid_from", default)]
    pub valid_from: String,
    #[serde(rename = "valid_until", default)]
    pub valid_until: String,
}
