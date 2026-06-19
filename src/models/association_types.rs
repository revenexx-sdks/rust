use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssociationTypes {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "is_quantified", default)]
    pub is_quantified: bool,
    #[serde(rename = "is_two_way", default)]
    pub is_two_way: bool,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
}
