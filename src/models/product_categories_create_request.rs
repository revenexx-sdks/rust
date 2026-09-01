use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductCategoriesCreateRequest {
    /// The category it is filed into. One row per (product, category), whichever
    /// way it got there.
    #[serde(rename = "category_id", default)]
    pub category_id: String,
    /// Sort order of this product inside the category.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The product filed into the category. Deleting the product deletes the
    /// membership with it.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// How the membership came about: 'manual' is hand-picked, 'rule' was
    /// materialized by a category rule. The two never touch each other — a
    /// recompute only ever inserts and deletes `rule` rows, so a hand-picked
    /// membership survives every pass.
    #[serde(rename = "source", default)]
    pub source: String,
}
