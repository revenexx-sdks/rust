use serde::{Deserialize, Serialize};

/// How price_snapshot_mode settled the two prices every line carries.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartConversionPricing {
    /// Lines in the cart when it converted.
    #[serde(rename = "lines", default)]
    pub lines: i64,
    /// Lines the mode had to rewrite because snapshot and unit_price disagreed —
    /// repriced in 'snapshot' mode, re-snapshotted in 'live' mode. A line whose
    /// snapshot carries no readable price is never touched in either mode.
    #[serde(rename = "lines_changed", default)]
    pub lines_changed: i64,
    /// The tenant's price_snapshot_mode, as it ran. 'snapshot' books the order on
    /// the price the buyer was shown; 'live' books it on the line's current
    /// unit_price and rewrites the snapshot to agree, so the frozen line never
    /// claims a price nobody was charged.
    #[serde(rename = "mode", default)]
    pub mode: String,
    /// The cart's frozen subtotal, and what the order is booked on.
    #[serde(rename = "subtotal_after", default)]
    pub subtotal_after: f64,
    /// The cart's subtotal as it stood before the mode was applied. Compare it
    /// with subtotal_after and 'why is the order €4 off the cart' is answered by
    /// the response instead of by an argument.
    #[serde(rename = "subtotal_before", default)]
    pub subtotal_before: f64,
}
