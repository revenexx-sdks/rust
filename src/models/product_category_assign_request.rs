use serde::{Deserialize, Serialize};

/// The category has to exist already; this route files a product into one, it
/// does not create one.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductCategoryAssignRequest {
    /// The category to file the product into.
    #[serde(rename = "category_id", default)]
    pub category_id: String,
    /// Sort order inside the category. Default 0.
    #[serde(rename = "position", default)]
    pub position: i64,
}
