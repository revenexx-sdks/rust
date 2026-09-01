use serde::{Deserialize, Serialize};

/// Rows this call added that were copied from nowhere, because the new market
/// would otherwise have been left unable to trade: the tenant
/// `fallback_locale` when neither market had a locale, and the base currency
/// when it is not in the copied set. Zero on both is the normal, healthy
/// answer — it means nothing had to be invented.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketCloneSeeded {
    /// 1 when the market's own base currency was registered because the copied set
    /// did not contain it; 0 otherwise.
    #[serde(rename = "currencies", default)]
    pub currencies: i64,
    /// 1 when the tenant's fallback_locale was written as this market's only
    /// locale, marked default; 0 otherwise.
    #[serde(rename = "locales", default)]
    pub locales: i64,
}
