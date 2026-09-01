use serde::{Deserialize, Serialize};

/// The exact-column filters this call was understood to carry, verbatim as
/// they arrived. A query parameter that is not a column of
/// `product_categories` — `?status=`, a typo, a filter another entity has
/// — is DROPPED and does not appear here, and the list comes back
/// unfiltered. This object is the only way to tell that apart from "nothing
/// matched".
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductCategoriesFilter {
    /// The literal `?category_id=` value this call was understood to carry.
    #[serde(rename = "category_id", default)]
    pub category_id: String,
    /// The literal `?created_at=` value this call was understood to carry.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The literal `?id=` value this call was understood to carry.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The literal `?position=` value this call was understood to carry.
    #[serde(rename = "position", default)]
    pub position: String,
    /// The literal `?product_id=` value this call was understood to carry.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// The literal `?source=` value this call was understood to carry.
    #[serde(rename = "source", default)]
    pub source: String,
}
