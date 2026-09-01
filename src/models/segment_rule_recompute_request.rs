use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SegmentRuleRecomputeRequest {
    /// Continuation token from a previous response — the id of the last
    /// organization the pass touched. Omit to resume or start automatically; pass
    /// null to force a restart from the beginning.
    #[serde(rename = "cursor", default)]
    pub cursor: String,
}
