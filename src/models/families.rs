use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Families {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "image_attribute", default)]
    pub image_attribute: String,
    #[serde(rename = "label_attribute", default)]
    pub label_attribute: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
