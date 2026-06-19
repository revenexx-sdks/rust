use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Menu {
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "created_by", default)]
    pub created_by: String,
    #[serde(rename = "deleted_at", default)]
    pub deleted_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "items", default)]
    pub items: serde_json::Value,
    #[serde(rename = "label", default)]
    pub label: String,
    #[serde(rename = "menu_key", default)]
    pub menu_key: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
