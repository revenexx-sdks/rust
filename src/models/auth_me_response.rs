use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthMeResponse {
    /// The customer record mirrored against this user, or null. A user with no
    /// contact resolves perfectly well — that is not the 404.
    #[serde(rename = "contact", default)]
    pub contact: crate::models::Contact,
    /// A contact's effective grants, derived from its role on every read —
    /// nothing here is stored, so a role change can never leave a stale grant
    /// behind. Null when there is no contact to derive them from.
    #[serde(rename = "permissions", default)]
    pub permissions: crate::models::ContactPermissions,
    /// The platform identity record, forwarded verbatim from the identity service.
    /// This app neither reshapes nor validates it, so treat unknown fields as
    /// forward-compatible; the ones named here are the ones this app itself writes
    /// and reads.
    #[serde(rename = "user", default)]
    pub user: serde_json::Value,
}
