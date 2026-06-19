use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReferenceEntityRecordsCreateRequest {
    #[serde(rename = "attribute_values", default)]
    pub attribute_values: serde_json::Value,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "reference_entity_id", default)]
    pub reference_entity_id: String,
}
