use serde::{Deserialize, Serialize};

/// The buyer context — restriction dimensions are ANDed, entries within a
/// dimension ORed, empty = unrestricted.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentEligibilityRequest {
    /// Order amount the fees are computed against (default 0).
    #[serde(rename = "amount", default)]
    pub amount: f64,
    /// Buyer ISO country code — methods with country restrictions need it.
    #[serde(rename = "country", default)]
    pub country: String,
    /// ISO 4217 code (default EUR).
    #[serde(rename = "currency", default)]
    pub currency: String,
}
