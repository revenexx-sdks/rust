use serde::{Deserialize, Serialize};

/// One organization inside one segment, and the record of how it got there
/// (hand-picked or matched by the rule).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SegmentMember {
    /// When the organization joined the segment.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Primary key of the membership row.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The member company. Segments group companies, never people — a person is
    /// reached through their organization.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    /// The segment.
    #[serde(rename = "segment_id", default)]
    pub segment_id: String,
    /// How this membership came about: 'manual' is hand-picked, 'rule' was
    /// materialized by a recompute. The distinction is load-bearing — a
    /// recompute only ever inserts and deletes 'rule' rows, so a hand-picked
    /// member survives every rule change.
    #[serde(rename = "source", default)]
    pub source: String,
    /// The tenant this row belongs to — the store slug, not an id. Set by the
    /// platform from the authenticated context, never by a caller; a write that
    /// carries it is ignored, and no request can read another tenant's rows by
    /// sending a different one.
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
}
