use serde::{Deserialize, Serialize};

/// No required fields — send {}.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegistrationApproveRequest {
    /// Who approved it — recorded on the contact and carried in the event. Free
    /// text (operator id or email); this app does not resolve it.
    #[serde(rename = "decided_by", default)]
    pub decided_by: String,
}
