use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Attributes {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "config", default)]
    pub config: serde_json::Value,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "entity_ref", default)]
    pub entity_ref: String,
    #[serde(rename = "entity_type", default)]
    pub entity_type: String,
    #[serde(rename = "group_id", default)]
    pub group_id: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "is_filterable", default)]
    pub is_filterable: bool,
    #[serde(rename = "is_unique", default)]
    pub is_unique: bool,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "localizable", default)]
    pub localizable: bool,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "scopable", default)]
    pub scopable: bool,
    #[serde(rename = "type", default)]
    pub xtype: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    #[serde(rename = "usable_in_grid", default)]
    pub usable_in_grid: bool,
    #[serde(rename = "validation", default)]
    pub validation: serde_json::Value,
}
