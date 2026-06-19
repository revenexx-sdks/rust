use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeOptionsUpdateRequest {
    #[serde(rename = "attribute_id", default)]
    pub attribute_id: String,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "swatch", default)]
    pub swatch: serde_json::Value,
}
