use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthRegisterResponse {
    /// True when the tenant runs registration_mode='approval_required' — do NOT
    /// log the buyer in.
    #[serde(rename = "approval_required", default)]
    pub approval_required: bool,
    /// The stored customer record — this app is its system of record.
    #[serde(rename = "contact", default)]
    pub contact: crate::models::Contact,
    /// 'pending' means the login is disabled until a merchant approves.
    #[serde(rename = "registration_status", default)]
    pub registration_status: String,
    /// The platform user that was created. Keep it: logout, /auth/me and the
    /// recovery confirm all take it.
    #[serde(rename = "user_id", default)]
    pub user_id: String,
    /// Whether an address confirmation went out. True only when the tenant's
    /// `email_verification` asks for one on registration, the registration is a
    /// finished account rather than an application, and `verification_url` was
    /// supplied.
    #[serde(rename = "verification_sent", default)]
    pub verification_sent: bool,
    /// Whether the tenant's welcome mail went out. Best effort on purpose: the
    /// account exists either way, and a registration is not undone because a
    /// message service was unreachable. False for an APPLICATION, which is not an
    /// account yet and is announced by `registration.submitted` instead.
    #[serde(rename = "welcome_sent", default)]
    pub welcome_sent: bool,
}
