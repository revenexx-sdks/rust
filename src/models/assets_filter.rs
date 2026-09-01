use serde::{Deserialize, Serialize};

/// The exact-column filters this call was understood to carry, verbatim as
/// they arrived. A query parameter that is not a column of `assets` —
/// `?status=`, a typo, a filter another entity has — is DROPPED and does not
/// appear here, and the list comes back unfiltered. This object is the only
/// way to tell that apart from "nothing matched".
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssetsFilter {
    /// The literal `?asset_family_id=` value this call was understood to carry.
    #[serde(rename = "asset_family_id", default)]
    pub asset_family_id: String,
    /// The literal `?attribute_values=` value this call was understood to carry.
    #[serde(rename = "attribute_values", default)]
    pub attribute_values: String,
    /// The literal `?code=` value this call was understood to carry.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The literal `?created_at=` value this call was understood to carry.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The literal `?delivery_path=` value this call was understood to carry.
    #[serde(rename = "delivery_path", default)]
    pub delivery_path: String,
    /// The literal `?external_url=` value this call was understood to carry.
    #[serde(rename = "external_url", default)]
    pub external_url: String,
    /// The literal `?id=` value this call was understood to carry.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The literal `?source=` value this call was understood to carry.
    #[serde(rename = "source", default)]
    pub source: String,
    /// The literal `?storage_asset_id=` value this call was understood to carry.
    #[serde(rename = "storage_asset_id", default)]
    pub storage_asset_id: String,
    /// The literal `?updated_at=` value this call was understood to carry.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
