use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "attachments", default)]
    pub attachments: Vec<serde_json::Value>,
    #[serde(rename = "attempts", default)]
    pub attempts: i64,
    #[serde(rename = "binding_id", default)]
    pub binding_id: String,
    #[serde(rename = "channel", default)]
    pub channel: String,
    #[serde(rename = "click_count", default)]
    pub click_count: i64,
    #[serde(rename = "clicked_at", default)]
    pub clicked_at: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "data", default)]
    pub data: Vec<serde_json::Value>,
    #[serde(rename = "delivered_at", default)]
    pub delivered_at: String,
    #[serde(rename = "error", default)]
    pub error: String,
    #[serde(rename = "from_draft", default)]
    pub from_draft: bool,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "idempotency_fingerprint", default)]
    pub idempotency_fingerprint: String,
    #[serde(rename = "idempotency_key", default)]
    pub idempotency_key: String,
    #[serde(rename = "locale", default)]
    pub locale: String,
    #[serde(rename = "market", default)]
    pub market: String,
    #[serde(rename = "message_class", default)]
    pub message_class: String,
    #[serde(rename = "open_count", default)]
    pub open_count: i64,
    #[serde(rename = "opened_at", default)]
    pub opened_at: String,
    #[serde(rename = "provider_message_id", default)]
    pub provider_message_id: String,
    #[serde(rename = "scheduled_for", default)]
    pub scheduled_for: String,
    #[serde(rename = "sent_at", default)]
    pub sent_at: String,
    #[serde(rename = "source_event_id", default)]
    pub source_event_id: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "subject", default)]
    pub subject: String,
    #[serde(rename = "suppression_reason", default)]
    pub suppression_reason: String,
    #[serde(rename = "template_key", default)]
    pub template_key: String,
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    #[serde(rename = "to", default)]
    pub to: String,
}
