use serde::{Deserialize, Serialize};

/// Everything but the code. Sending a different code is a 400 rather than a
/// silent no-op: renaming it would orphan every row that carries it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingServiceLevelUpdateRequest {
    /// The sentence under the title, explaining when to pick this service level.
    /// Null when the title says enough.
    #[serde(rename = "description", default)]
    pub description: String,
    /// Localized descriptions. A flat map keyed by locale — the Cockpit falls
    /// back to `en`. Null means the row has no translations and every client shows
    /// the untranslated column instead.
    #[serde(rename = "descriptions", default)]
    pub descriptions: serde_json::Value,
    /// Promote this value; the previous default is demoted. POST …/make-default
    /// does the same thing without an edit.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Localized titles. A flat map keyed by locale — the Cockpit falls back to
    /// `en`. Null means the row has no translations and every client shows the
    /// untranslated column instead.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Sort order in a select — the collection is returned in it.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// What an operator reads in a select. The name a merchant renames; the code
    /// underneath never moves.
    #[serde(rename = "title", default)]
    pub title: String,
    /// Semantic badge colour for a UI listing the set. The client owns what each
    /// tone looks like.
    #[serde(rename = "tone", default)]
    pub tone: String,
}
