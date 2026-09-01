use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoryRecomputeResult {
    /// Membership rows inserted with source='rule' by this call.
    #[serde(rename = "added", default)]
    pub added: i64,
    /// False → the bulk insert was refused and the call fell back to one request
    /// per row. A performance fact, not an error.
    #[serde(rename = "batched", default)]
    pub batched: bool,
    /// The category this pass belongs to, echoed back — a caller driving several
    /// loops keys its state by it.
    #[serde(rename = "category_id", default)]
    pub category_id: String,
    /// When the pass completed, and what `categories.rules_computed_at` was
    /// stamped with. Null while `done` is false.
    #[serde(rename = "computed_at", default)]
    pub computed_at: String,
    /// The product id this call reconciled up to, to hand back on the next one.
    /// Null when `done`.
    #[serde(rename = "cursor", default)]
    pub cursor: String,
    /// False → this call spent its budget mid-pass. Send `cursor` back to
    /// continue; the counters below are THIS call only, so a caller looping to
    /// completion sums them itself.
    #[serde(rename = "done", default)]
    pub done: bool,
    /// Matching products examined by this call.
    #[serde(rename = "processed", default)]
    pub processed: i64,
    /// Stale rule rows deleted by this call.
    #[serde(rename = "removed", default)]
    pub removed: i64,
    /// Products the rule currently selects. Null while `done` is false — the
    /// pass has not seen the whole catalog yet, so there is no total to report.
    #[serde(rename = "total", default)]
    pub total: i64,
}
