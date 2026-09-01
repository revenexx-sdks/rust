use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Layout {
    #[serde(rename = "color_accent", default)]
    pub color_accent: String,
    #[serde(rename = "color_bg", default)]
    pub color_bg: String,
    #[serde(rename = "color_text", default)]
    pub color_text: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    #[serde(rename = "font_family", default)]
    pub font_family: String,
    #[serde(rename = "footer_note", default)]
    pub footer_note: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    #[serde(rename = "legal_name", default)]
    pub legal_name: String,
    #[serde(rename = "lifecycle_state", default)]
    pub lifecycle_state: String,
    #[serde(rename = "logo_url", default)]
    pub logo_url: String,
    #[serde(rename = "markets", default)]
    pub markets: Vec<serde_json::Value>,
    #[serde(rename = "menu_links", default)]
    pub menu_links: Vec<serde_json::Value>,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "postal_address", default)]
    pub postal_address: String,
    #[serde(rename = "sender_name", default)]
    pub sender_name: String,
    #[serde(rename = "social_links", default)]
    pub social_links: Vec<serde_json::Value>,
    #[serde(rename = "support_email", default)]
    pub support_email: String,
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    #[serde(rename = "valid_from", default)]
    pub valid_from: String,
    #[serde(rename = "valid_until", default)]
    pub valid_until: String,
    #[serde(rename = "width", default)]
    pub width: String,
}
