use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReferenceEntityRecordsUpdateRequest {
    #[serde(rename = "attribute_values", default)]
    pub attribute_values: serde_json::Value,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "reference_entity_id", default)]
    pub reference_entity_id: String,
}
