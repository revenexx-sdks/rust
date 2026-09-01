use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SegmentRulePreviewRequest {
    /// The conditions, combined by `rule_match`. At least one, at most 25.
    #[serde(rename = "conditions", default)]
    pub conditions: Vec<crate::models::SegmentRuleCondition>,
    /// How the conditions combine. Default 'all'.
    #[serde(rename = "rule_match", default)]
    pub rule_match: String,
    /// Only 'organizations' is supported; any other value is rejected. A segment
    /// groups COMPANIES — the people are reached through them.
    #[serde(rename = "target", default)]
    pub target: String,
}
