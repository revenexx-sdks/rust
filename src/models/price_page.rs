use serde::{Deserialize, Serialize};

/// Where this page sits in the full result set. Rows beyond `limit` are not
/// returned and are not lost — ask for the next page with `offset`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PricePage {
    /// true when `offset + returned < total` — there is another page to fetch.
    #[serde(rename = "hasMore", default)]
    pub has_more: bool,
    /// Page size actually applied — the `limit` you sent, clamped to 1…200
    /// (default 50).
    #[serde(rename = "limit", default)]
    pub limit: i64,
    /// Row offset actually applied (default 0).
    #[serde(rename = "offset", default)]
    pub offset: i64,
    /// Rows in `items` on this page.
    #[serde(rename = "returned", default)]
    pub returned: i64,
    /// Rows matching the filter across all pages, not just this one.
    #[serde(rename = "total", default)]
    pub total: i64,
}
