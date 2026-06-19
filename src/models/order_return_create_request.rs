use serde::{Deserialize, Serialize};

/// Register a return against the shipped quantities — the return number is
/// drawn from the 'return' range.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderReturnCreateRequest {
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "positions", default)]
    pub positions: Vec<crate::models::OrderReturnPosition>,
    #[serde(rename = "reason", default)]
    pub reason: String,
}
