use serde::{Deserialize, Serialize};

/// Metric
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metric {
    /// The date at which this metric was aggregated in ISO 8601 format.
    #[serde(rename = "date", default)]
    pub date: String,
    /// The value of this metric at the timestamp.
    #[serde(rename = "value", default)]
    pub value: i64,
}
