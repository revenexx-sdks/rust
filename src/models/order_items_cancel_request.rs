use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderItemsCancelRequest {
    /// Acting user/system.
    #[serde(rename = "cancelled_by", default)]
    pub cancelled_by: String,
    #[serde(rename = "positions", default)]
    pub positions: Vec<crate::models::OrderCancelPosition>,
    #[serde(rename = "reason", default)]
    pub reason: String,
}
