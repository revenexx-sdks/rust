use serde::{Deserialize, Serialize};

/// Creates AND authorizes: self-managed methods authorize immediately, PSP
/// methods may answer next_action (redirect). Eligibility is re-checked
/// server-side.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentCreateRequest {
    /// Order amount — 0 is legal (free orders), negative is not.
    #[serde(rename = "amount", default)]
    pub amount: f64,
    /// The cart this payment pays for.
    #[serde(rename = "cart_id", default)]
    pub cart_id: String,
    /// Paying customer contact.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// Buyer ISO country code for the eligibility check.
    #[serde(rename = "country", default)]
    pub country: String,
    /// ISO 4217 code (default EUR).
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// Same key answers the same payment instead of a duplicate.
    #[serde(rename = "idempotency_key", default)]
    pub idempotency_key: String,
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Code of a configured payment method.
    #[serde(rename = "method_code", default)]
    pub method_code: String,
    /// External order reference — also the webhook fallback key.
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
    /// Where the PSP redirect flow returns the buyer to.
    #[serde(rename = "return_url", default)]
    pub return_url: String,
}
