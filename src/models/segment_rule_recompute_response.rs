use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SegmentRuleRecomputeResponse {
    /// Rule memberships inserted by THIS call.
    #[serde(rename = "added", default)]
    pub added: i64,
    /// True when every membership insert used a bulk array request; false if any
    /// batch fell back to row-at-a-time.
    #[serde(rename = "batched", default)]
    pub batched: bool,
    /// Set when the pass completes.
    #[serde(rename = "computed_at", default)]
    pub computed_at: String,
    /// Send back on the next call; null when the pass is done.
    #[serde(rename = "cursor", default)]
    pub cursor: String,
    /// False means work remains — POST again with `cursor`.
    #[serde(rename = "done", default)]
    pub done: bool,
    /// Matching organizations examined by THIS call.
    #[serde(rename = "processed", default)]
    pub processed: i64,
    /// Rule memberships deleted by THIS call.
    #[serde(rename = "removed", default)]
    pub removed: i64,
    /// The segment that was recomputed.
    #[serde(rename = "segment_id", default)]
    pub segment_id: String,
    /// The rule's full match count; null until done.
    #[serde(rename = "total", default)]
    pub total: i64,
}
