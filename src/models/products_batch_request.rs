use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductsBatchRequest {
    #[serde(rename = "ids", default)]
    pub ids: Vec<String>,
    #[serde(rename = "skus", default)]
    pub skus: Vec<String>,
}
