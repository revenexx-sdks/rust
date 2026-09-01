use serde::{Deserialize, Serialize};

/// A record of what was taken off an order and why — either the whole order
/// (while nothing had shipped) or named quantities off a partly shipped one.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderCancellation {
    /// Who cancelled, as the caller reported it — an operator, a desk, a system.
    /// Free text; this app does not resolve it against a user directory.
    #[serde(rename = "cancelled_by", default)]
    pub cancelled_by: String,
    /// When the cancellation was recorded.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Primary key of the cancellation record.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The order that was cancelled from.
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    /// What this record removed. A scope 'order' record carries every position in
    /// full; a scope 'items' record carries exactly the quantities that were
    /// named.
    #[serde(rename = "positions", default)]
    pub positions: Vec<crate::models::OrderCancellationPosition>,
    /// Why it was cancelled, free text. Mandatory when the tenant sets
    /// cancel_requires_reason — for those merchants an unexplained cancellation
    /// is refused with a 400.
    #[serde(rename = "reason", default)]
    pub reason: String,
    /// Which of the two cancellations this was: 'order' is the full cancel (only
    /// possible while nothing has shipped, and it cancels every position in full),
    /// 'items' is the quantity-based one that takes open quantities off a partly
    /// shipped order.
    #[serde(rename = "scope", default)]
    pub scope: String,
}
