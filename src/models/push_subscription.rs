use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushSubscription {
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "endpoint", default)]
    pub endpoint: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "last_seen_at", default)]
    pub last_seen_at: String,
    #[serde(rename = "subscriber_id", default)]
    pub subscriber_id: String,
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    #[serde(rename = "user_agent", default)]
    pub user_agent: String,
}
