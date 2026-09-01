use serde::{Deserialize, Serialize};

/// Just enough of a published page to link to it. The block tree is not here
/// — fetch it with `GET /pages/delivery/page`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeliveryPageRef {
    /// The page type, so a sitemap can group or a picker can filter.
    #[serde(rename = "bundle", default)]
    pub bundle: String,
    /// The page id, usable as `?id=` on the delivery route.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The path segment to build the URL from. `null` for a page reachable only by
    /// id, which a sitemap should skip.
    #[serde(rename = "slug", default)]
    pub slug: String,
    /// The page title in its source language — this projection is not
    /// language-resolved.
    #[serde(rename = "title", default)]
    pub title: String,
}
