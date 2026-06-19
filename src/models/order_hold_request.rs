use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderHoldRequest {
    /// Why the order is blocked (shown on the shipping guard).
    #[serde(rename = "reason", default)]
    pub reason: String,
}
