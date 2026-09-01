use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LibraryTemplate {
    #[serde(rename = "body_html", default)]
    pub body_html: String,
    #[serde(rename = "body_text", default)]
    pub body_text: String,
    #[serde(rename = "channel", default)]
    pub channel: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "design", default)]
    pub design: Vec<serde_json::Value>,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "key", default)]
    pub key: String,
    #[serde(rename = "locale", default)]
    pub locale: String,
    #[serde(rename = "subject", default)]
    pub subject: String,
    #[serde(rename = "suggested_event", default)]
    pub suggested_event: String,
    #[serde(rename = "suggested_recipient", default)]
    pub suggested_recipient: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    #[serde(rename = "variables", default)]
    pub variables: Vec<serde_json::Value>,
}
