use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IoProfileUpdateRequest {
    /// Default 'insert'.
    #[serde(rename = "apply_mode", default)]
    pub apply_mode: String,
    #[serde(rename = "direction", default)]
    pub direction: String,
    /// Default 'carts'.
    #[serde(rename = "entity", default)]
    pub entity: String,
    /// Default 'json'.
    #[serde(rename = "format", default)]
    pub format: String,
    #[serde(rename = "is_template", default)]
    pub is_template: bool,
    /// Column mapping (Baseline-IO-compatible).
    #[serde(rename = "mapping", default)]
    pub mapping: serde_json::Value,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "options", default)]
    pub options: serde_json::Value,
}
