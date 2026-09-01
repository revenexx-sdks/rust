use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthOtpConfirmResponse {
    /// The customer record behind the login, or null when none is mirrored yet.
    #[serde(rename = "contact", default)]
    pub contact: crate::models::Contact,
    /// A contact's effective grants, derived from its role on every read —
    /// nothing here is stored, so a role change can never leave a stale grant
    /// behind. Null when there is no contact to derive them from.
    #[serde(rename = "permissions", default)]
    pub permissions: crate::models::ContactPermissions,
    /// Platform auth session. Treat `secret` as a credential — the trusted BFF
    /// stores it server-side (HTTP-only cookie), never in the browser.
    #[serde(rename = "session", default)]
    pub session: crate::models::AuthSession,
}
