use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReorderScan {
    /// One entry per published event, in the order they went out. Re-running the
    /// scan on the same day returns the SAME ids and publishes nothing a second
    /// time — the event id is derived from the row and the day, and the bus
    /// drops the repeat.
    #[serde(rename = "emitted", default)]
    pub emitted: Vec<crate::models::ReorderScanEmit>,
    /// false when reorder_alert_enabled is off — nothing was published, and not
    /// because nothing is low.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// How many rows were at or below their point when the scan ran.
    #[serde(rename = "scanned", default)]
    pub scanned: i64,
}
