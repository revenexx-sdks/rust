use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderEvent {
    #[serde(rename = "actor", default)]
    pub actor: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "payload", default)]
    pub payload: serde_json::Value,
}
