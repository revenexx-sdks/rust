use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderNumberRangeUpdateRequest {
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    /// Range key drawn by the app ('order', 'delivery', 'return') — unique per
    /// tenant.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Current counter value (default 0) — the next number draws counter+step.
    #[serde(rename = "counter", default)]
    pub counter: i64,
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Zero-padding width of the counter (default 6).
    #[serde(rename = "padding", default)]
    pub padding: i64,
    /// Position numbering increment for order items (default 10).
    #[serde(rename = "position_step", default)]
    pub position_step: i64,
    /// Default ''.
    #[serde(rename = "prefix", default)]
    pub prefix: String,
    /// Counter increment per drawn number (default 1).
    #[serde(rename = "step", default)]
    pub step: i64,
    /// Default ''.
    #[serde(rename = "suffix", default)]
    pub suffix: String,
}
