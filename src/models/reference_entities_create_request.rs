use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReferenceEntitiesCreateRequest {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "image", default)]
    pub image: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
}
