use serde::{Deserialize, Serialize};

/// One method as a checkout should render it: identity, wording, and what it
/// costs this buyer.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EligiblePaymentMethod {
    /// The code to send back as `method_code` when the payment is created.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The currency `fee` is in — the one the request asked with, echoed.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// The merchant's line about this method, to show beside it at checkout.
    #[serde(rename = "description", default)]
    pub description: String,
    /// The surcharge this method costs THIS buyer, already computed against the
    /// requested amount — a fixed fee as it stands, a percentage resolved into
    /// an amount. Not a column: no CHECK bounds it, so none is declared.
    #[serde(rename = "fee", default)]
    pub fee: f64,
    /// How `fee` was arrived at, for a checkout that wants to show "2 % surcharge"
    /// rather than the amount.
    #[serde(rename = "fee_type", default)]
    pub fee_type: String,
    /// Whether choosing this method starts a PSP flow ('psp') or authorizes
    /// immediately ('self_managed').
    #[serde(rename = "kind", default)]
    pub kind: String,
    /// Buyer-facing names keyed by language tag, or null when the merchant
    /// configured none — then `name` is all there is.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// The operator-facing name. Prefer `labels` for anything a buyer reads.
    #[serde(rename = "name", default)]
    pub name: String,
    /// The merchant's sort order. The list is already sorted by it; it is carried
    /// so a client that re-sorts can put it back.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The PSP behind it, for a checkout that has to load a provider SDK before it
    /// can collect an instrument. null for self-managed methods.
    #[serde(rename = "provider", default)]
    pub provider: String,
}
