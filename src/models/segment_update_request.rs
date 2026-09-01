use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SegmentUpdateRequest {
    /// Stable identifier, unique per tenant — what other apps and integrations
    /// name the segment by. Free text, but lowercase with underscores is the
    /// convention every seeded vocabulary follows.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Localized display names keyed by language tag. Null means nobody translated
    /// it and a client falls back to showing the code.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Sort order in the cockpit, ascending. Ties fall back to insertion order.
    /// Default 0.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// How the conditions combine: 'all' (default) is AND, 'any' is OR. Null means
    /// the same as 'all'.
    #[serde(rename = "rule_match", default)]
    pub rule_match: String,
    /// The selector that decides membership, stored verbatim. Null means the
    /// segment is manual-only. The same rule language product categories use,
    /// evaluated over organization columns, `setting:<key>` entries and the
    /// organization_metrics projection — so 'no order in 365 days' is
    /// expressible without joining the orders app. Null makes the segment
    /// manual-only. Changing it does not move a single membership — run the
    /// recompute.
    #[serde(rename = "rules", default)]
    pub rules: crate::models::SegmentRules,
}
