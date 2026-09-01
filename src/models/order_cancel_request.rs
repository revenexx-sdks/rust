use serde::{Deserialize, Serialize};

/// Cancels the WHOLE order, and only while nothing has shipped. Both fields
/// are optional unless the tenant requires a reason.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderCancelRequest {
    /// Who cancelled, as the caller reported it — an operator, a desk, a system.
    /// Free text; this app does not resolve it against a user directory.
    #[serde(rename = "cancelled_by", default)]
    pub cancelled_by: String,
    /// Why it was cancelled, free text. Mandatory when the tenant sets
    /// cancel_requires_reason — for those merchants an unexplained cancellation
    /// is refused with a 400.
    #[serde(rename = "reason", default)]
    pub reason: String,
}
