use serde::{Deserialize, Serialize};

/// The selector that decides membership, stored verbatim. Null means the
/// segment is manual-only. The same rule language product categories use,
/// evaluated over organization columns, `setting:<key>` entries and the
/// organization_metrics projection — so 'no order in 365 days' is
/// expressible without joining the orders app. Null makes the segment
/// manual-only. Changing it does not move a single membership — run the
/// recompute.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SegmentRules {
    /// The conditions, combined by `rule_match`. At least one, at most 25.
    #[serde(rename = "conditions", default)]
    pub conditions: Vec<crate::models::SegmentRuleCondition>,
    /// Only 'organizations' is supported; any other value is rejected. A segment
    /// groups COMPANIES — the people are reached through them.
    #[serde(rename = "target", default)]
    pub target: String,
}
