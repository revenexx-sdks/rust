use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IoProfile {
    #[serde(rename = "apply_mode", default)]
    pub apply_mode: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "direction", default)]
    pub direction: String,
    #[serde(rename = "entity", default)]
    pub entity: String,
    #[serde(rename = "format", default)]
    pub format: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "is_template", default)]
    pub is_template: bool,
    #[serde(rename = "mapping", default)]
    pub mapping: serde_json::Value,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "options", default)]
    pub options: serde_json::Value,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
