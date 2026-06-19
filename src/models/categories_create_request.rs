use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoriesCreateRequest {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "parent_id", default)]
    pub parent_id: String,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "values", default)]
    pub values: serde_json::Value,
}
