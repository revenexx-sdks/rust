use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingRateTier {
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "from_value", default)]
    pub from_value: f64,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "method_id", default)]
    pub method_id: String,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "price", default)]
    pub price: f64,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
