use serde::{Deserialize, Serialize};

/// The exact-column filters this call was understood to carry, verbatim as
/// they arrived. A query parameter that is not a column of
/// `measurement_families` — `?status=`, a typo, a filter another entity has
/// — is DROPPED and does not appear here, and the list comes back
/// unfiltered. This object is the only way to tell that apart from "nothing
/// matched".
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeasurementFamiliesFilter {
    /// The literal `?code=` value this call was understood to carry.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The literal `?created_at=` value this call was understood to carry.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The literal `?id=` value this call was understood to carry.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The literal `?labels=` value this call was understood to carry.
    #[serde(rename = "labels", default)]
    pub labels: String,
    /// The literal `?standard_unit=` value this call was understood to carry.
    #[serde(rename = "standard_unit", default)]
    pub standard_unit: String,
    /// The literal `?units=` value this call was understood to carry.
    #[serde(rename = "units", default)]
    pub units: String,
    /// The literal `?updated_at=` value this call was understood to carry.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
