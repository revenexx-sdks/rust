use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssociationTypesCreateRequest {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "is_quantified", default)]
    pub is_quantified: bool,
    #[serde(rename = "is_two_way", default)]
    pub is_two_way: bool,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
}
