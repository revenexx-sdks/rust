use serde::{Deserialize, Serialize};

/// Where this page sits in the result set. Everything needed to fetch the next
/// one is here, so a client never has to guess whether it has seen everything.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormsPage {
    /// True while `offset + returned < total`: another page follows, at `offset +
    /// returned`.
    #[serde(rename = "hasMore", default)]
    pub has_more: bool,
    /// The page size that was applied — the `limit` parameter after clamping to
    /// 1…200, or 50 when none was given.
    #[serde(rename = "limit", default)]
    pub limit: i64,
    /// How many matching rows were skipped before this page.
    #[serde(rename = "offset", default)]
    pub offset: i64,
    /// How many rows are in `items` — below `limit` exactly on the last page.
    #[serde(rename = "returned", default)]
    pub returned: i64,
    /// How many rows match the filter in total, ignoring the page. This is the
    /// number to show a merchant; `returned` is only what fitted.
    #[serde(rename = "total", default)]
    pub total: i64,
}
