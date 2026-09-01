use serde::{Deserialize, Serialize};

/// Where this answer sits in the whole result set.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderPage {
    /// Whether another page exists after this one (offset + returned < total). The
    /// one field a "load more" button should read.
    #[serde(rename = "hasMore", default)]
    pub has_more: bool,
    /// The page size that was applied. A requested limit above 200 is CLAMPED to
    /// 200 rather than refused, so this is the number to believe, not the one you
    /// sent.
    #[serde(rename = "limit", default)]
    pub limit: i64,
    /// The row offset that was applied.
    #[serde(rename = "offset", default)]
    pub offset: i64,
    /// How many rows are in `items` right here — less than `limit` on the last
    /// page.
    #[serde(rename = "returned", default)]
    pub returned: i64,
    /// How many rows match the filter in total, ignoring limit and offset. This is
    /// what a page count is computed from.
    #[serde(rename = "total", default)]
    pub total: i64,
}
