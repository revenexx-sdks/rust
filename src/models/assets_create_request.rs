use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssetsCreateRequest {
    #[serde(rename = "asset_family_id", default)]
    pub asset_family_id: String,
    #[serde(rename = "attribute_values", default)]
    pub attribute_values: serde_json::Value,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "media_uuid", default)]
    pub media_uuid: String,
}
