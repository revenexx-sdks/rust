use serde::{Deserialize, Serialize};

/// Child rows copied in from the source, per collection — only codes this
/// market did not already carry. Zero everywhere on a second run: the call is
/// idempotent.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketBackfillAdded {
    /// Traded currencies added from the source market.
    #[serde(rename = "currencies", default)]
    pub currencies: i64,
    /// Locales added from the source market.
    #[serde(rename = "locales", default)]
    pub locales: i64,
    /// Tax classes added from the source market.
    #[serde(rename = "tax_classes", default)]
    pub tax_classes: i64,
}
