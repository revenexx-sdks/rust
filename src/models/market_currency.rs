use serde::{Deserialize, Serialize};

/// One currency a market accepts, as opposed to the single base currency on
/// the market row that its prices are quoted in. The base currency must be
/// registered here or the market cannot serve.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketCurrency {
    /// ISO 4217 code, unique per market — one entry in the set of currencies
    /// this market TRADES in, as opposed to the single base currency on the market
    /// row that its prices are quoted in. The base currency must appear here or
    /// the market cannot serve; clone and backfill register it for you.
    #[serde(rename = "code", default)]
    pub code: String,
    /// When the currency was registered on this market. Set by the database; never
    /// writable.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Primary key of this currency registration. The currency is named by `code`
    /// everywhere else.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The currency offered first to a buyer who states no preference. At most one
    /// per market, and it should be the market's base currency — readiness
    /// reports it as a warning when it is not.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// The market this currency belongs to. Filled from the route path on write
    /// and never read out of the body; ON DELETE CASCADE, so deleting the market
    /// deletes this row.
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    /// Sort position among this market's currencies, ascending, default 0 — the
    /// order a currency switcher lists them in.
    #[serde(rename = "position", default)]
    pub position: i64,
}
