use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SegmentMemberUpdateRequest {
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
    /// member survives every rule change. Default 'manual'.
    #[serde(rename = "source", default)]
    pub source: String,
}
