use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductCategoriesCreateRequest {
    #[serde(rename = "category_id", default)]
    pub category_id: String,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
}
