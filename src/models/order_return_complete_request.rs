use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderReturnCompleteRequest {
    /// How the return was settled (refund, replacement, …).
    #[serde(rename = "resolution", default)]
    pub resolution: String,
}
