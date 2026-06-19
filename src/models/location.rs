use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "address", default)]
    pub address: serde_json::Value,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "priority", default)]
    pub priority: i64,
    #[serde(rename = "type", default)]
    pub xtype: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
