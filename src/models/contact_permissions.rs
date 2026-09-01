use serde::{Deserialize, Serialize};

/// A contact's effective grants, derived from its role on every read —
/// nothing here is stored, so a role change can never leave a stale grant
/// behind. Carried here so a BFF does not need a second call to decide what to
/// render.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactPermissions {
    /// False while the contact is blocked or its registration is still
    /// pending/rejected — it holds the role but must not act on it.
    #[serde(rename = "active", default)]
    pub active: bool,
    /// The person these grants belong to. Null when the answer describes nobody
    /// — a user with no contact mirrored against it.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// Amount ceiling in the market's currency; null means no ceiling. Only
    /// meaningful together with the 'orders.approve' permission.
    #[serde(rename = "order_approval_limit", default)]
    pub order_approval_limit: f64,
    /// The organization the role applies inside. Null for a standalone (B2C)
    /// contact — a role with no company to hold it in.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    /// What this role may do. Derived from the role — see GET /customers/roles.
    #[serde(rename = "permissions", default)]
    pub permissions: Vec<String>,
    /// The role this contact holds in its organization, and the only input to
    /// `permissions`.
    #[serde(rename = "role", default)]
    pub role: String,
}
