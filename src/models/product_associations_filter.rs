use serde::{Deserialize, Serialize};

/// The exact-column filters this call was understood to carry, verbatim as
/// they arrived. A query parameter that is not a column of
/// `product_associations` — `?status=`, a typo, a filter another entity has
/// — is DROPPED and does not appear here, and the list comes back
/// unfiltered. This object is the only way to tell that apart from "nothing
/// matched".
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductAssociationsFilter {
    /// The literal `?association_type_id=` value this call was understood to
    /// carry.
    #[serde(rename = "association_type_id", default)]
    pub association_type_id: String,
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
    /// The literal `?quantity=` value this call was understood to carry.
    #[serde(rename = "quantity", default)]
    pub quantity: String,
    /// The literal `?target_product_id=` value this call was understood to carry.
    #[serde(rename = "target_product_id", default)]
    pub target_product_id: String,
}
