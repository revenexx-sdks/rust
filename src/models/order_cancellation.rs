use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderCancellation {
    #[serde(rename = "cancelled_by", default)]
    pub cancelled_by: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "positions", default)]
    pub positions: serde_json::Value,
    #[serde(rename = "reason", default)]
    pub reason: String,
    #[serde(rename = "scope", default)]
    pub scope: String,
}
