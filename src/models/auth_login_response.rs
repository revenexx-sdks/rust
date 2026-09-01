use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthLoginResponse {
    /// The challenge to answer, when one was required. Send it back as
    /// `challenge_id`.
    #[serde(rename = "challenge_id", default)]
    pub challenge_id: String,
    /// The customer record behind the login. Null when a platform user has no
    /// contact mirrored against it — a storefront should treat that as "signed
    /// in, but not a customer of this app".
    #[serde(rename = "contact", default)]
    pub contact: crate::models::Contact,
    /// Present and true when the tenant's `mfa_mode` is 'required'. The password
    /// was one of two things this buyer has to prove: a challenge has already been
    /// created and mailed, and the session above must NOT be treated as signed in
    /// until `PUT /customers/auth/mfa/challenge` confirms the code. The session
    /// travels anyway because answering needs it — the expected caller holds
    /// session material server-side, and this is the point at which that trust is
    /// used.
    #[serde(rename = "mfa_required", default)]
    pub mfa_required: bool,
    /// A contact's effective grants, derived from its role on every read —
    /// nothing here is stored, so a role change can never leave a stale grant
    /// behind. Carried here so a BFF does not need a second call to decide what to
    /// render.
    #[serde(rename = "permissions", default)]
    pub permissions: crate::models::ContactPermissions,
    /// Platform auth session. Treat `secret` as a credential — the trusted BFF
    /// stores it server-side (HTTP-only cookie), never in the browser.
    #[serde(rename = "session", default)]
    pub session: crate::models::AuthSession,
}
