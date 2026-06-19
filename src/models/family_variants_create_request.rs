use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FamilyVariantsCreateRequest {
    #[serde(rename = "axes", default)]
    pub axes: serde_json::Value,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "family_id", default)]
    pub family_id: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
}
