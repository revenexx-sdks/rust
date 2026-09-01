use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeasurementFamiliesCreateRequest {
    /// The measurement family's stable identifier. A `measure` attribute names one
    /// and then offers that family's units.
    #[serde(rename = "code", default)]
    pub code: String,
    /// What the measurement family is called, per language tag.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// The unit every value of this family is converted to before it is compared
    /// or sorted — the unit each `convert_factor` is relative to.
    #[serde(rename = "standard_unit", default)]
    pub standard_unit: String,
    /// The units this family offers. `convert_factor` multiplies a value into
    /// `standard_unit`, so a gram is 0.001 kilograms; `symbol` is what a form
    /// prints next to the number.
    #[serde(rename = "units", default)]
    pub units: serde_json::Value,
}
