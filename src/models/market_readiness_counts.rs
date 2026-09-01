use serde::{Deserialize, Serialize};

/// How much of a market this market actually is. All three at zero is a market
/// that is a row and nothing else — the state two of the three live markets
/// on the platform were left in, and the reason /clone and /backfill exist.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketReadinessCounts {
    /// Traded currencies registered on this market.
    #[serde(rename = "currencies", default)]
    pub currencies: i64,
    /// Locales registered on this market.
    #[serde(rename = "locales", default)]
    pub locales: i64,
    /// Tax classes registered on this market.
    #[serde(rename = "tax_classes", default)]
    pub tax_classes: i64,
}
