use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeOptions {
    #[serde(rename = "attribute_id", default)]
    pub attribute_id: String,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "swatch", default)]
    pub swatch: serde_json::Value,
}
