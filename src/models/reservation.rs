use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Reservation {
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "expires_at", default)]
    pub expires_at: String,
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
    #[serde(rename = "sku", default)]
    pub sku: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
