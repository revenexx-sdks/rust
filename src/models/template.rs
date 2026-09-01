use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Template {
    #[serde(rename = "body_html", default)]
    pub body_html: String,
    #[serde(rename = "body_text", default)]
    pub body_text: String,
    #[serde(rename = "channel", default)]
    pub channel: String,
    #[serde(rename = "content_sid", default)]
    pub content_sid: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "design", default)]
    pub design: Vec<serde_json::Value>,
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    #[serde(rename = "has_unpublished_changes", default)]
    pub has_unpublished_changes: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "is_published", default)]
    pub is_published: String,
    #[serde(rename = "key", default)]
    pub key: String,
    #[serde(rename = "layout_id", default)]
    pub layout_id: String,
    #[serde(rename = "lifecycle_state", default)]
    pub lifecycle_state: String,
    #[serde(rename = "locale", default)]
    pub locale: String,
    #[serde(rename = "markets", default)]
    pub markets: Vec<serde_json::Value>,
    #[serde(rename = "message_class", default)]
    pub message_class: String,
    #[serde(rename = "published_version_id", default)]
    pub published_version_id: String,
    #[serde(rename = "source_library_key", default)]
    pub source_library_key: String,
    #[serde(rename = "subject", default)]
    pub subject: String,
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    #[serde(rename = "test_mode", default)]
    pub test_mode: bool,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    #[serde(rename = "uses_raw_html", default)]
    pub uses_raw_html: String,
    #[serde(rename = "valid_from", default)]
    pub valid_from: String,
    #[serde(rename = "valid_until", default)]
    pub valid_until: String,
    #[serde(rename = "variable_defaults", default)]
    pub variable_defaults: Vec<serde_json::Value>,
    #[serde(rename = "variables", default)]
    pub variables: Vec<serde_json::Value>,
    #[serde(rename = "whatsapp_category", default)]
    pub whatsapp_category: String,
}
