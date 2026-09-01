use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReorderScanEmit {
    /// The event id on the bus. Stable per (row, day), which is what makes a
    /// re-run harmless.
    #[serde(rename = "event_id", default)]
    pub event_id: String,
    /// The stock row the event is about.
    #[serde(rename = "stock_level_id", default)]
    pub stock_level_id: String,
}
