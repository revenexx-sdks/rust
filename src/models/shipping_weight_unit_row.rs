use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingWeightUnitRow {
    /// What a rate request names in `weight_unit`, and what a market's
    /// `weight_unit` setting stores. Immutable once created — renaming it would
    /// orphan every row carrying it.
    #[serde(rename = "code", default)]
    pub code: String,
    /// When the row was created (UTC).
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The sentence under the title, explaining when to pick this weight unit.
    /// Null when the title says enough.
    #[serde(rename = "description", default)]
    pub description: String,
    /// Localized descriptions. A flat map keyed by locale — the Cockpit falls
    /// back to `en`. Null means the row has no translations and every client shows
    /// the untranslated column instead.
    #[serde(rename = "descriptions", default)]
    pub descriptions: serde_json::Value,
    /// How many BASE units (kilograms) one of this unit weighs — a tonne is
    /// 1000, a gram 0.001, a pound 0.45359237. This number prices parcels: every
    /// weight matrix converts a request through it. Must be > 0; the base unit is
    /// fixed at 1 and rejects a change.
    #[serde(rename = "factor", default)]
    pub factor: f64,
    /// Row id, assigned by the database on insert.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The anchor every other factor is expressed in. Exactly one row, fixed at
    /// install, not writable and not deletable — moving it would silently
    /// reprice every weight matrix.
    #[serde(rename = "is_base", default)]
    pub is_base: bool,
    /// The unit a market whose `weight_unit` setting is unset keys its tiers in.
    /// Exactly one row carries it.
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
