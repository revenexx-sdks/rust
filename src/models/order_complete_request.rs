use serde::{Deserialize, Serialize};

/// No required fields — send {}.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderCompleteRequest {
    /// Who closed the order, as the caller reports it. Not stored on the order: it
    /// is carried in the order.completed event's payload, which is where the audit
    /// trail keeps who did what. Free text, not resolved against a user directory.
    #[serde(rename = "completed_by", default)]
    pub completed_by: String,
}
