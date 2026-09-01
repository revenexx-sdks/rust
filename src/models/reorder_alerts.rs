use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReorderAlerts {
    /// The rows at or below their reorder point, worst first (by `shortfall`).
    /// Computed on read, so it is never stale — and never empty because of
    /// caching: an empty list means nothing is low, unless `enabled` is false.
    #[serde(rename = "alerts", default)]
    pub alerts: Vec<crate::models::ReorderAlert>,
    /// false when reorder_alert_enabled is off — the list is then empty by
    /// policy, not because nothing is low.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// The threshold applied to rows carrying none of their own.
    #[serde(rename = "reorder_point_default", default)]
    pub reorder_point_default: f64,
}
