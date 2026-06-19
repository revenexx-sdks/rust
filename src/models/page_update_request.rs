use serde::{Deserialize, Serialize};

/// Partial update — only title, slug, status, meta and bundle are applied;
/// other keys are ignored.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageUpdateRequest {
    #[serde(rename = "bundle", default)]
    pub bundle: String,
    #[serde(rename = "meta", default)]
    pub meta: serde_json::Value,
    #[serde(rename = "slug", default)]
    pub slug: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "title", default)]
    pub title: String,
}
