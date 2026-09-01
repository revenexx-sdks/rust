use serde::{Deserialize, Serialize};

/// The delivery window a checkout can print. Calendar days, cut-off evaluated
/// in UTC (send `at` to control the instant).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingDeliveryEstimate {
    /// Whether the cut-off had passed at evaluation time, costing a day.
    #[serde(rename = "cutoff_passed", default)]
    pub cutoff_passed: bool,
    /// The cut-off applied (HH:MM, UTC), or null when none is configured — the
    /// carrier's own when it declares one, else the market's `cutoff_time`
    /// setting.
    #[serde(rename = "cutoff_time", default)]
    pub cutoff_time: String,
    /// ship_date + eta_days_min.
    #[serde(rename = "earliest", default)]
    pub earliest: String,
    /// The tenant's handling_days setting, as applied.
    #[serde(rename = "handling_days", default)]
    pub handling_days: i64,
    /// ship_date + eta_days_max.
    #[serde(rename = "latest", default)]
    pub latest: String,
    /// The day the parcel leaves — today plus handling days, plus one when the
    /// cut-off has passed.
    #[serde(rename = "ship_date", default)]
    pub ship_date: String,
}
