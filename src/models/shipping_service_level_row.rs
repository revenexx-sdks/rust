use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingServiceLevelRow {
    /// What `shipping_carriers.service_level` stores. Immutable once created —
    /// renaming it would orphan every row carrying it.
    #[serde(rename = "code", default)]
    pub code: String,
    /// When the row was created (UTC).
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The sentence under the title, explaining when to pick this service level.
    /// Null when the title says enough.
    #[serde(rename = "description", default)]
    pub description: String,
    /// Localized descriptions. A flat map keyed by locale — the Cockpit falls
    /// back to `en`. Null means the row has no translations and every client shows
    /// the untranslated column instead.
    #[serde(rename = "descriptions", default)]
    pub descriptions: serde_json::Value,
    /// Row id, assigned by the database on insert.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The service level a fallback lands on. Exactly one row carries it, and POST
    /// …/make-default is what moves it.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Seeded on install rather than typed by the merchant. Still renameable and
    /// still deletable; it only says where the row came from.
    #[serde(rename = "is_system", default)]
    pub is_system: bool,
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
    /// When the row was last written (UTC).
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
