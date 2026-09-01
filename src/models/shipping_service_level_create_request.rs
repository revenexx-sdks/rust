use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingServiceLevelCreateRequest {
    /// Lowercase letters, digits, - or _, starting with a letter. What
    /// `shipping_carriers.service_level` stores. Immutable once created —
    /// renaming it would orphan every row carrying it.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The sentence under the title, explaining when to pick this service level.
    /// Null when the title says enough.
    #[serde(rename = "description", default)]
    pub description: String,
    /// Localized descriptions. A flat map keyed by locale — the Cockpit falls
    /// back to `en`. Null means the row has no translations and every client shows
    /// the untranslated column instead.
    #[serde(rename = "descriptions", default)]
    pub descriptions: serde_json::Value,
    /// Promote this value on creation; the previous default is demoted.
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
