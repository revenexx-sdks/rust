use serde::{Deserialize, Serialize};

/// The exact-column filters this call was understood to carry, verbatim as
/// they arrived. A query parameter that is not a column of `attributes` —
/// `?status=`, a typo, a filter another entity has — is DROPPED and does not
/// appear here, and the list comes back unfiltered. This object is the only
/// way to tell that apart from "nothing matched".
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributesFilter {
    /// The literal `?code=` value this call was understood to carry.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The literal `?config=` value this call was understood to carry.
    #[serde(rename = "config", default)]
    pub config: String,
    /// The literal `?created_at=` value this call was understood to carry.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The literal `?entity_ref=` value this call was understood to carry.
    #[serde(rename = "entity_ref", default)]
    pub entity_ref: String,
    /// The literal `?entity_type=` value this call was understood to carry.
    #[serde(rename = "entity_type", default)]
    pub entity_type: String,
    /// The literal `?group_id=` value this call was understood to carry.
    #[serde(rename = "group_id", default)]
    pub group_id: String,
    /// The literal `?id=` value this call was understood to carry.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The literal `?is_filterable=` value this call was understood to carry.
    #[serde(rename = "is_filterable", default)]
    pub is_filterable: String,
    /// The literal `?is_unique=` value this call was understood to carry.
    #[serde(rename = "is_unique", default)]
    pub is_unique: String,
    /// The literal `?labels=` value this call was understood to carry.
    #[serde(rename = "labels", default)]
    pub labels: String,
    /// The literal `?localizable=` value this call was understood to carry.
    #[serde(rename = "localizable", default)]
    pub localizable: String,
    /// The literal `?position=` value this call was understood to carry.
    #[serde(rename = "position", default)]
    pub position: String,
    /// The literal `?scopable=` value this call was understood to carry.
    #[serde(rename = "scopable", default)]
    pub scopable: String,
    /// The literal `?type=` value this call was understood to carry.
    #[serde(rename = "type", default)]
    pub xtype: String,
    /// The literal `?updated_at=` value this call was understood to carry.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    /// The literal `?usable_in_grid=` value this call was understood to carry.
    #[serde(rename = "usable_in_grid", default)]
    pub usable_in_grid: String,
    /// The literal `?validation=` value this call was understood to carry.
    #[serde(rename = "validation", default)]
    pub validation: String,
}
