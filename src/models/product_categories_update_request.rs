use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductCategoriesUpdateRequest {
    #[serde(rename = "category_id", default)]
    pub category_id: String,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
}
