use serde::{Deserialize, Serialize};

/// The exact-column filters this call was understood to carry, verbatim as
/// they arrived. A query parameter that is not a column of `locations` — a
/// typo, a filter another entity has, `?q=` — is DROPPED and cannot appear
/// here, and the list comes back unfiltered. This object is the only way to
/// tell that apart from "nothing matched".
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocationsFilter {
    /// The literal `?address=` value this call was understood to carry.
    #[serde(rename = "address", default)]
    pub address: String,
    /// The literal `?code=` value this call was understood to carry.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The literal `?created_at=` value this call was understood to carry.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The literal `?enabled=` value this call was understood to carry.
    #[serde(rename = "enabled", default)]
    pub enabled: String,
    /// The literal `?id=` value this call was understood to carry.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The literal `?labels=` value this call was understood to carry.
    #[serde(rename = "labels", default)]
    pub labels: String,
    /// The literal `?metadata=` value this call was understood to carry.
    #[serde(rename = "metadata", default)]
    pub metadata: String,
    /// The literal `?name=` value this call was understood to carry.
    #[serde(rename = "name", default)]
    pub name: String,
    /// The literal `?priority=` value this call was understood to carry.
    #[serde(rename = "priority", default)]
    pub priority: String,
    /// The literal `?type=` value this call was understood to carry.
    #[serde(rename = "type", default)]
    pub xtype: String,
    /// The literal `?updated_at=` value this call was understood to carry.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
