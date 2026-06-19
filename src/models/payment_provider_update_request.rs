use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentProviderUpdateRequest {
    /// PSP credentials — the catalog's credential_fields say which keys the auth
    /// scheme expects.
    #[serde(rename = "credentials", default)]
    pub credentials: serde_json::Value,
    /// Only enabled providers transact (default false).
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// Display name — defaults to the catalog label.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Free-form provider options.
    #[serde(rename = "options", default)]
    pub options: serde_json::Value,
    /// Provider code — must exist in the catalog (GET
    /// /payments/providers/catalog).
    #[serde(rename = "provider", default)]
    pub provider: String,
    /// Sandbox/test credentials (default true).
    #[serde(rename = "test_mode", default)]
    pub test_mode: bool,
    /// Shared secret for PSP callback verification.
    #[serde(rename = "webhook_secret", default)]
    pub webhook_secret: String,
}
