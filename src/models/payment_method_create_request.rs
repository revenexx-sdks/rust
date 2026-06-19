use serde::{Deserialize, Serialize};

/// A method needs its identity: code + name.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentMethodCreateRequest {
    /// Stable method code (unique per tenant, e.g. 'invoice', 'card').
    #[serde(rename = "code", default)]
    pub code: String,
    /// Allowed ISO country codes — empty/omitted = unrestricted.
    #[serde(rename = "countries", default)]
    pub countries: Vec<String>,
    #[serde(rename = "description", default)]
    pub description: String,
    /// Disabled methods are never eligible (default false).
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// Fixed amount or percent value, per fee_type (default 0).
    #[serde(rename = "fee_amount", default)]
    pub fee_amount: f64,
    /// ISO 4217 code (default EUR).
    #[serde(rename = "fee_currency", default)]
    pub fee_currency: String,
    /// How 'fee_amount' applies (default 'none').
    #[serde(rename = "fee_type", default)]
    pub fee_type: String,
    /// Self-managed (merchant fulfils, default) or PSP-backed ('provider' required
    /// to transact).
    #[serde(rename = "kind", default)]
    pub kind: String,
    /// Localized display names ({ de, en, … }).
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Maximum order amount — omitted = no upper bound.
    #[serde(rename = "max_order_value", default)]
    pub max_order_value: f64,
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Minimum order amount — omitted = no lower bound.
    #[serde(rename = "min_order_value", default)]
    pub min_order_value: f64,
    /// Display name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sort position in the checkout (default 0).
    #[serde(rename = "position", default)]
    pub position: i64,
    /// PSP code from the catalog — only for kind 'psp'.
    #[serde(rename = "provider", default)]
    pub provider: String,
    /// The provider's payment method id (e.g. 'card', 'paypal').
    #[serde(rename = "provider_method", default)]
    pub provider_method: String,
}
