use serde::{Deserialize, Serialize};

/// One publication of this page, without the snapshot — who published, when,
/// and under what name.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageRevisionRef {
    /// When this revision was published.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The user id that published.
    #[serde(rename = "created_by", default)]
    pub created_by: String,
    /// That user's display name, copied in at publish time so the history stays
    /// readable after the user is gone.
    #[serde(rename = "created_by_name", default)]
    pub created_by_name: String,
    /// The revision id. A page's `published_revision_id` points at one of these,
    /// and it is the only thing delivery reads.
    #[serde(rename = "id", default)]
    pub id: String,
    /// What this publication was called, e.g. "Autumn campaign". It is what turns
    /// the history into a list of changes rather than a list of timestamps.
    #[serde(rename = "label", default)]
    pub label: String,
    /// The page this revision belongs to.
    #[serde(rename = "page_id", default)]
    pub page_id: String,
}
