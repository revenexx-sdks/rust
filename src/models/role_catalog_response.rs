use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoleCatalogResponse {
    /// The built-in permission vocabulary, one entry per grant. The authoritative,
    /// installed-app-aware list is the platform's permission ledger — this app
    /// deliberately does not duplicate it.
    #[serde(rename = "permissions", default)]
    pub permissions: Vec<serde_json::Value>,
    /// Every role a contact of this tenant can hold, least to most privileged.
    #[serde(rename = "roles", default)]
    pub roles: Vec<serde_json::Value>,
    /// 'tenant' — the configured mapping answered. 'defaults' — this tenant
    /// has no roles yet, or custom_roles_enabled locks the ledger, and the
    /// built-ins answered.
    #[serde(rename = "source", default)]
    pub source: String,
}
