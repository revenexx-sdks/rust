use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Comment {
    #[serde(rename = "author_id", default)]
    pub author_id: String,
    #[serde(rename = "author_name", default)]
    pub author_name: String,
    #[serde(rename = "block_uuids", default)]
    pub block_uuids: serde_json::Value,
    #[serde(rename = "body", default)]
    pub body: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "page_id", default)]
    pub page_id: String,
    #[serde(rename = "parent_id", default)]
    pub parent_id: String,
    #[serde(rename = "resolved", default)]
    pub resolved: bool,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
