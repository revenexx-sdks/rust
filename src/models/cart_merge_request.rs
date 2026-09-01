use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartMergeRequest {
    /// The cart being folded in. It must be active, and it does NOT survive as a
    /// workspace: its lines are copied into the target, it becomes status merged,
    /// and merged_into_cart_id points at the target. Its own lines stay on it as
    /// the record of what was moved.
    #[serde(rename = "source_cart_id", default)]
    pub source_cart_id: String,
    /// The cart that SURVIVES. Must be active; it gains the source's lines
    /// (identical product lines at the same price adding up) and its totals are
    /// recomputed.
    #[serde(rename = "target_cart_id", default)]
    pub target_cart_id: String,
}
