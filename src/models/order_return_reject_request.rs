use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderReturnRejectRequest {
    /// Fallback for 'resolution'.
    #[serde(rename = "reason", default)]
    pub reason: String,
    /// Why the return was rejected.
    #[serde(rename = "resolution", default)]
    pub resolution: String,
}
