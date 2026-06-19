use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Assets {
    #[serde(rename = "asset_family_id", default)]
    pub asset_family_id: String,
    #[serde(rename = "attribute_values", default)]
    pub attribute_values: serde_json::Value,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "media_uuid", default)]
    pub media_uuid: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
