use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductLabelsRequest {
    /// Product ids to name. At most 500.
    #[serde(rename = "ids", default)]
    pub ids: Vec<String>,
    /// Product SKUs to name. At most 500.
    #[serde(rename = "skus", default)]
    pub skus: Vec<String>,
}
