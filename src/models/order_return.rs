use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderReturn {
    #[serde(rename = "completed_at", default)]
    pub completed_at: String,
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
    #[serde(rename = "positions", default)]
    pub positions: serde_json::Value,
    #[serde(rename = "reason", default)]
    pub reason: String,
    #[serde(rename = "received_at", default)]
    pub received_at: String,
    #[serde(rename = "registered_at", default)]
    pub registered_at: String,
    #[serde(rename = "rejected_at", default)]
    pub rejected_at: String,
    #[serde(rename = "resolution", default)]
    pub resolution: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
