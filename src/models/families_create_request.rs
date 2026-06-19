use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FamiliesCreateRequest {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "image_attribute", default)]
    pub image_attribute: String,
    #[serde(rename = "label_attribute", default)]
    pub label_attribute: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
}
