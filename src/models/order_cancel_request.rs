use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderCancelRequest {
    /// Acting user/system.
    #[serde(rename = "cancelled_by", default)]
    pub cancelled_by: String,
    #[serde(rename = "reason", default)]
    pub reason: String,
}
