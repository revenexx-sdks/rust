use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderReturnRejectRequest {
    /// Free-text fallback for 'resolution' — a sentence about this one return,
    /// not a value out of the set.
    #[serde(rename = "reason", default)]
    pub reason: String,
    /// Why the return was refused.
    #[serde(rename = "resolution", default)]
    pub resolution: String,
}
