use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentProvider {
    /// When this PSP was configured for the tenant.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Only an enabled provider takes NEW payments: a method pointing at a
    /// disabled one falls through to the tenant's `fallback_provider`, and to a
    /// 422 if there is none. Nothing else reads it — capture, cancel and refund
    /// on the payments this PSP already holds go on working — which is what
    /// makes disabling the safe retirement and deleting the refused one.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// Id of the PSP configuration row — what the provider routes address. The
    /// provider itself is named by `provider`.
    #[serde(rename = "id", default)]
    pub id: String,
    /// Operator-facing name of the configuration. Defaults to the catalog label,
    /// and is worth changing when a tenant runs two accounts with one PSP.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Per-provider switches this app understands, plus anything the merchant
    /// keeps beside them. Three keys are the app's own: `logo_url` (the bundled
    /// logo, filled in when the provider is seeded), `capture_method` and
    /// `three_ds` (what the prism driver does today). Free jsonb — an unknown
    /// key is stored and ignored.
    #[serde(rename = "options", default)]
    pub options: serde_json::Value,
    /// The catalog code of the PSP this row configures — one row per provider
    /// per tenant. GET /payments/providers/catalog lists every code that may
    /// appear here. It is what every payment and every method naming this PSP
    /// resolves it by, so changing it is refused with 409 for as long as one of
    /// them does.
    #[serde(rename = "provider", default)]
    pub provider: String,
    /// Whether the driver talks to the PSP's sandbox. New configurations start in
    /// test mode: a provider nobody verified must not touch live money.
    #[serde(rename = "test_mode", default)]
    pub test_mode: bool,
    /// When its configuration last changed — including a credential rotation,
    /// which is otherwise invisible from the outside.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
