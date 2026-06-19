use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MenuUpdateRequest {
    #[serde(rename = "items", default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "label", default)]
    pub label: String,
}
