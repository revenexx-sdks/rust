use serde::{Deserialize, Serialize};

/// Number pattern: '{prefix}{counter padded to padding}{suffix}'.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderNumberRangeCreateRequest {
    /// The sales channel this range was created for, as a label. It does NOT
    /// select the range: a draw finds the range by `code` alone, and the unique
    /// index (tenant, code) means one code is one range per tenant — so an order
    /// on another channel draws from the same range this one names. Null on the
    /// three seeded ranges, which is every tenant-wide range.
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    /// Which counter this is, in the app's own words: 'order' numbers orders,
    /// 'delivery' numbers delivery notes, 'return' numbers returns. Unique per
    /// tenant, and the value the order_number_range_code /
    /// delivery_number_range_code / return_number_range_code settings point at —
    /// a setting naming a code no range carries is the 422 'number_range_missing'.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The last number DRAWN — state, not configuration. The next draw is
    /// counter + step and writes the new value back, so moving this forward skips
    /// numbers and moving it back re-issues them (and the unique index then
    /// answers 409). Defaults to 0, so the first number drawn is step.
    #[serde(rename = "counter", default)]
    pub counter: i64,
    /// Free-form data for the caller. This app stores it and returns it, and reads
    /// nothing out of it.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// How wide the counter is written, zero-padded: 6 makes 123 into 000123. 0
    /// writes the bare number. Widening it later does not renumber what was
    /// already drawn. Defaults to 6.
    #[serde(rename = "padding", default)]
    pub padding: i64,
    /// The gap between the position numbers of a new order: 10 numbers the lines
    /// 10, 20, 30 — room to slot a line in between later without renumbering the
    /// rest. Read from the ORDER range only. Defaults to 10.
    #[serde(rename = "position_step", default)]
    pub position_step: i64,
    /// Literal text in front of the counter: 'ORD-' turns counter 123 into
    /// ORD-000123. Empty by default. Defaults to ''.
    #[serde(rename = "prefix", default)]
    pub prefix: String,
    /// How far the counter moves per draw. 1 is consecutive numbering; a larger
    /// step is what a merchant chooses who does not want their order volume
    /// readable off an invoice. Defaults to 1.
    #[serde(rename = "step", default)]
    pub step: i64,
    /// Literal text after the counter — a market or year marker on merchants who
    /// number that way. Empty by default, which is what most of them use. Defaults
    /// to ''.
    #[serde(rename = "suffix", default)]
    pub suffix: String,
}
