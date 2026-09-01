use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Binding {
    #[serde(rename = "channel", default)]
    pub channel: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    #[serde(rename = "event_topic", default)]
    pub event_topic: String,
    #[serde(rename = "fallback_order", default)]
    pub fallback_order: i64,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "locale", default)]
    pub locale: String,
    #[serde(rename = "recipient", default)]
    pub recipient: String,
    #[serde(rename = "template_key", default)]
    pub template_key: String,
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
