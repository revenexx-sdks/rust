use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegistrationRejectRequest {
    /// Who rejected it — recorded on the contact and carried in the event.
    #[serde(rename = "decided_by", default)]
    pub decided_by: String,
    /// Why the application was declined. Always stored on the contact. It only
    /// reaches the APPLICANT when the tenant's registration_reason_disclosed
    /// setting is on — the event payload then carries it, and so does the 403
    /// the login answers.
    #[serde(rename = "reason", default)]
    pub reason: String,
}
