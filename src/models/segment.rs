use serde::{Deserialize, Serialize};

/// A named group of ORGANIZATIONS — by hand, by rule, or both at once.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Segment {
    /// Stable identifier, unique per tenant — what other apps and integrations
    /// name the segment by. Free text, but lowercase with underscores is the
    /// convention every seeded vocabulary follows.
    #[serde(rename = "code", default)]
    pub code: String,
    /// When the segment was created.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Primary key of the segment.
    #[serde(rename = "id", default)]
    pub id: String,
    /// Localized display names keyed by language tag. Null means nobody translated
    /// it and a client falls back to showing the code.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Sort order in the cockpit, ascending. Ties fall back to insertion order.
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
    /// expressible without joining the orders app.
    #[serde(rename = "rules", default)]
    pub rules: serde_json::Value,
    /// When the rule last finished a COMPLETE recompute. Null after a rule change,
    /// and while a chunked recompute is still running — so it doubles as "are
    /// the rule memberships trustworthy right now?".
    #[serde(rename = "rules_computed_at", default)]
    pub rules_computed_at: String,
    /// The tenant this row belongs to — the store slug, not an id. Set by the
    /// platform from the authenticated context, never by a caller; a write that
    /// carries it is ignored, and no request can read another tenant's rows by
    /// sending a different one.
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    /// When any column of this row last changed.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
