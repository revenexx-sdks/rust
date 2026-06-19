use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReferenceEntitiesUpdateRequest {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "image", default)]
    pub image: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
}
