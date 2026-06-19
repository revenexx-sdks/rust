use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeasurementFamiliesCreateRequest {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "standard_unit", default)]
    pub standard_unit: String,
    #[serde(rename = "units", default)]
    pub units: serde_json::Value,
}
