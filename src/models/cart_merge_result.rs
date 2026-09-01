use serde::{Deserialize, Serialize};

/// Which cart survived, and what it cost. `target` is the cart that SURVIVES,
/// already recomputed — that is the one to render. The source cart still
/// exists and still holds its own lines: a merge copies them into the target
/// and closes the source, it does not move them.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartMergeResult {
    /// The source cart, now status merged, with merged_into_cart_id pointing at
    /// the target. It still exists and still holds its own lines: the merge
    /// copies, it does not move.
    #[serde(rename = "merged_cart_id", default)]
    pub merged_cart_id: String,
    /// Lines read out of the source. Identical product lines at the same price add
    /// up rather than duplicating, so the target may have gained fewer rows than
    /// this.
    #[serde(rename = "merged_lines", default)]
    pub merged_lines: i64,
    #[serde(rename = "target", default)]
    pub target: crate::models::Cart,
}
