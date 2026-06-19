use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Template {
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "created_by", default)]
    pub created_by: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "field_name", default)]
    pub field_name: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    #[serde(rename = "label", default)]
    pub label: String,
    #[serde(rename = "page_bundle", default)]
    pub page_bundle: String,
    #[serde(rename = "tree", default)]
    pub tree: serde_json::Value,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
