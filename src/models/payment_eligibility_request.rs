use serde::{Deserialize, Serialize};

/// The buyer context — restriction dimensions are ANDed, entries within a
/// dimension ORed, empty = unrestricted.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentEligibilityRequest {
    /// The order amount the order-value bounds are checked against and the
    /// percentage fees are computed from. Defaults to 0, which excludes every
    /// method carrying a minimum. Nothing is written, so the ledger's own amount
    /// bound does not apply here.
    #[serde(rename = "amount", default)]
    pub amount: f64,
    /// The buyer's ISO 3166-1 alpha-2 country code. A method restricted to
    /// countries is excluded without it — an unknown buyer sees only the
    /// unrestricted methods, which is the safe default and not a bug.
    #[serde(rename = "country", default)]
    pub country: String,
    /// ISO 4217 code the amount is in, echoed onto every computed fee. Defaults to
    /// EUR. This app does no conversion: the fee comes back in the currency it was
    /// asked with.
    #[serde(rename = "currency", default)]
    pub currency: String,
}
