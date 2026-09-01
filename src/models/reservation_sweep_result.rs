use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReservationSweepResult {
    /// How many active reservations were found past their hold: the ones with an
    /// `expires_at` in the past, plus the undated ones older than their market's
    /// TTL.
    #[serde(rename = "expired", default)]
    pub expired: i64,
    /// The market codes this run had to resolve a window for — every market that
    /// had an undated active reservation. Empty when nothing is market-assigned,
    /// which is the usual case.
    #[serde(rename = "markets", default)]
    pub markets: Vec<String>,
    /// How many were actually given back — `reserved` lowered on the stock row
    /// and a `release` booking written for each. It equals `expired` unless a row
    /// vanished mid-run. Idempotent: a second run immediately after finds nothing
    /// and answers 0.
    #[serde(rename = "released", default)]
    pub released: i64,
    /// The cut-off this run used — everything whose hold had run out by this
    /// moment was released. It is the run's own clock, not a stored value.
    #[serde(rename = "swept_at", default)]
    pub swept_at: String,
    /// The `reservation_ttl_minutes` that applied to reservations belonging to NO
    /// market — the tenant baseline. A reservation assigned to a market is
    /// judged against that market's own window instead, which is why this is
    /// reported rather than assumed to be the only one.
    #[serde(rename = "ttl_minutes", default)]
    pub ttl_minutes: f64,
}
