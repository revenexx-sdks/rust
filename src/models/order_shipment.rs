use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderShipment {
    #[serde(rename = "carrier", default)]
    pub carrier: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "number", default)]
    pub number: String,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "shipped_at", default)]
    pub shipped_at: String,
    #[serde(rename = "tracking_code", default)]
    pub tracking_code: String,
    #[serde(rename = "tracking_url", default)]
    pub tracking_url: String,
}
