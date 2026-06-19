use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cart {
    #[serde(rename = "abandoned_at", default)]
    pub abandoned_at: String,
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "currency", default)]
    pub currency: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "is_current", default)]
    pub is_current: bool,
    #[serde(rename = "item_count", default)]
    pub item_count: i64,
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    #[serde(rename = "merged_into_cart_id", default)]
    pub merged_into_cart_id: String,
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
    #[serde(rename = "ordered_at", default)]
    pub ordered_at: String,
    #[serde(rename = "session_key", default)]
    pub session_key: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "subtotal", default)]
    pub subtotal: f64,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
