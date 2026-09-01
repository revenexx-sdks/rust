use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderItemsCancelRequest {
    /// Who cancelled, as the caller reported it — an operator, a desk, a system.
    /// Free text; this app does not resolve it against a user directory.
    #[serde(rename = "cancelled_by", default)]
    pub cancelled_by: String,
    /// The quantities to take off the order. Required here, unlike on /ship and
    /// /return: cancelling everything by default is not a thing anybody should be
    /// able to do by omission — that is what /cancel is for.
    #[serde(rename = "positions", default)]
    pub positions: Vec<crate::models::OrderCancelPosition>,
    /// Why it was cancelled, free text. Mandatory when the tenant sets
    /// cancel_requires_reason — for those merchants an unexplained cancellation
    /// is refused with a 400.
    #[serde(rename = "reason", default)]
    pub reason: String,
}
