use serde::{Deserialize, Serialize};

/// Where in the result set this answer sits. `limit` and `offset` are the
/// values that were APPLIED, not the ones that were asked for — the data
/// plane clamps rather than refuses, so an out-of-range or unparseable value
/// comes back corrected here instead of as a 400.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketsPage {
    /// True when `offset + returned < total`, i.e. another page exists. Cheaper to
    /// branch on than comparing the three numbers yourself.
    #[serde(rename = "hasMore", default)]
    pub has_more: bool,
    /// Page size actually applied. A request over 200 is clamped to 200, one under
    /// 1 (or one that is not a number) to the 50-row default.
    #[serde(rename = "limit", default)]
    pub limit: i64,
    /// Row offset actually applied. A negative offset is clamped to 0.
    #[serde(rename = "offset", default)]
    pub offset: i64,
    /// Rows in `items` on this page. Lower than `limit` on the last page.
    #[serde(rename = "returned", default)]
    pub returned: i64,
    /// Rows matching the filter across ALL pages, ignoring limit and offset —
    /// the number to paginate against.
    #[serde(rename = "total", default)]
    pub total: i64,
}
