use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketCurrencyUpdateRequest {
    /// ISO 4217 code, unique per market — one entry in the set of currencies
    /// this market TRADES in, as opposed to the single base currency on the market
    /// row that its prices are quoted in. The base currency must appear here or
    /// the market cannot serve; clone and backfill register it for you.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The currency offered first to a buyer who states no preference. At most one
    /// per market, and it should be the market's base currency — readiness
    /// reports it as a warning when it is not.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Sort position among this market's currencies, ascending, default 0 — the
    /// order a currency switcher lists them in.
    #[serde(rename = "position", default)]
    pub position: i64,
}
