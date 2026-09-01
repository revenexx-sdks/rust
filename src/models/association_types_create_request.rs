use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssociationTypesCreateRequest {
    /// The kind of relation between two products. Unique per tenant.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Declares that a relation of this kind carries a quantity — a bundle, a
    /// bill of materials. `product_associations.quantity` is where that number
    /// goes, and it is meaningless without this flag.
    #[serde(rename = "is_quantified", default)]
    pub is_quantified: bool,
    /// Declares the relation symmetric — an accessory of A is an accessory of B.
    /// It is a declaration a client reads: this app stores one row per direction
    /// and does not create the mirror for you.
    #[serde(rename = "is_two_way", default)]
    pub is_two_way: bool,
    /// What the relation is called in a product form, per language tag.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
}
