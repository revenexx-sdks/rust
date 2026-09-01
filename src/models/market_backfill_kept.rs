use serde::{Deserialize, Serialize};

/// What this market already held BEFORE the repair, per collection — the
/// rows that were left exactly as the merchant left them.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketBackfillKept {
    /// Traded currencies this market already held, untouched.
    #[serde(rename = "currencies", default)]
    pub currencies: i64,
    /// Locales this market already held, untouched.
    #[serde(rename = "locales", default)]
    pub locales: i64,
    /// Tax classes this market already held, untouched.
    #[serde(rename = "tax_classes", default)]
    pub tax_classes: i64,
}
