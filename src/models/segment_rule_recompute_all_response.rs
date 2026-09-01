use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SegmentRuleRecomputeAllResponse {
    /// Rule memberships inserted across every segment in THIS call.
    #[serde(rename = "added", default)]
    pub added: i64,
    /// False when any segment is unfinished or skipped — call again.
    #[serde(rename = "done", default)]
    pub done: bool,
    /// Segments whose own recompute raised — they carry `error` and `status` in
    /// `segments` and did not abort the run.
    #[serde(rename = "failed", default)]
    pub failed: i64,
    /// Ruled segments the run looked at.
    #[serde(rename = "processed", default)]
    pub processed: i64,
    /// Rule memberships deleted across every segment in THIS call.
    #[serde(rename = "removed", default)]
    pub removed: i64,
    /// One entry per segment; a failed segment carries `error` and `status`
    /// instead of the counters.
    #[serde(rename = "segments", default)]
    pub segments: Vec<serde_json::Value>,
    /// Segments the budget did not reach at all.
    #[serde(rename = "skipped", default)]
    pub skipped: i64,
}
