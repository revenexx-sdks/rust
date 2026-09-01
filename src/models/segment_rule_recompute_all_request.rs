use serde::{Deserialize, Serialize};

/// No parameters — send {}.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SegmentRuleRecomputeAllRequest {
}
