use serde::{Deserialize, Serialize};

/// What to record about this publication.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PagePublishRequest {
    /// Publish despite violations. Without it a page with unresolved violations
    /// answers 422 and nothing is written.
    #[serde(rename = "force", default)]
    pub force: bool,
    /// What to call this publication in the page's history — "Autumn campaign"
    /// rather than a timestamp.
    #[serde(rename = "label", default)]
    pub label: String,
}
