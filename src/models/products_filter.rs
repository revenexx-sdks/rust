use serde::{Deserialize, Serialize};

/// The exact-column filters this call was understood to carry, verbatim as
/// they arrived. A query parameter that is not a column of `products` —
/// `?status=`, a typo, a filter another entity has — is DROPPED and does not
/// appear here, and the list comes back unfiltered. This object is the only
/// way to tell that apart from "nothing matched".
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductsFilter {
    /// The literal `?attribute_values=` value this call was understood to carry.
    #[serde(rename = "attribute_values", default)]
    pub attribute_values: String,
    /// The literal `?completeness=` value this call was understood to carry.
    #[serde(rename = "completeness", default)]
    pub completeness: String,
    /// The literal `?created_at=` value this call was understood to carry.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The literal `?deleted_at=` value this call was understood to carry.
    #[serde(rename = "deleted_at", default)]
    pub deleted_at: String,
    /// The literal `?enabled=` value this call was understood to carry.
    #[serde(rename = "enabled", default)]
    pub enabled: String,
    /// The literal `?family_id=` value this call was understood to carry.
    #[serde(rename = "family_id", default)]
    pub family_id: String,
    /// The literal `?family_variant_id=` value this call was understood to carry.
    #[serde(rename = "family_variant_id", default)]
    pub family_variant_id: String,
    /// The literal `?id=` value this call was understood to carry.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The literal `?kind=` value this call was understood to carry.
    #[serde(rename = "kind", default)]
    pub kind: String,
    /// The literal `?label=` value this call was understood to carry.
    #[serde(rename = "label", default)]
    pub label: String,
    /// The literal `?parent_id=` value this call was understood to carry.
    #[serde(rename = "parent_id", default)]
    pub parent_id: String,
    /// The literal `?quantified_associations=` value this call was understood to
    /// carry.
    #[serde(rename = "quantified_associations", default)]
    pub quantified_associations: String,
    /// The literal `?sku=` value this call was understood to carry.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// The literal `?tax_class=` value this call was understood to carry.
    #[serde(rename = "tax_class", default)]
    pub tax_class: String,
    /// The literal `?updated_at=` value this call was understood to carry.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
