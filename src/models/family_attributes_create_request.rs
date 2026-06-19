use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FamilyAttributesCreateRequest {
    #[serde(rename = "attribute_id", default)]
    pub attribute_id: String,
    #[serde(rename = "family_id", default)]
    pub family_id: String,
    #[serde(rename = "is_required", default)]
    pub is_required: bool,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "required_channels", default)]
    pub required_channels: serde_json::Value,
}
