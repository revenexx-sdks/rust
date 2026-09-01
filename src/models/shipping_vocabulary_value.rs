use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingVocabularyValue {
    /// What the value means. Either one string or a locale map keyed by locale
    /// (e.g. {en, de}) — curated copy carries the map, a value falling back to
    /// its own key carries the string.
    #[serde(rename = "description", default)]
    pub description: String,
    /// Table-backed only: localized descriptions, keyed by locale.
    #[serde(rename = "descriptions", default)]
    pub descriptions: serde_json::Value,
    /// weight-units only: kilograms per unit. A weight vocabulary without it is a
    /// list of names you cannot convert with.
    #[serde(rename = "factor", default)]
    pub factor: f64,
    /// The value ends the lifecycle.
    #[serde(rename = "final", default)]
    pub xfinal: bool,
    /// weight-units only: the unit every other factor is expressed in.
    #[serde(rename = "is_base", default)]
    pub is_base: bool,
    /// Table-backed only: the value a caller falls back to, so a client can mark
    /// it without reading the settings as well.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Table-backed only: seeded on install. Still renameable and retirable.
    #[serde(rename = "is_system", default)]
    pub is_system: bool,
    /// The value as the database stores it — what a column carries and what a
    /// filter matches. The only field a machine should compare on.
    #[serde(rename = "key", default)]
    pub key: String,
    /// Table-backed only: localized titles, keyed by locale. Absent for a
    /// vocabulary whose values come from a CHECK constraint — those carry their
    /// copy in `title` instead.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// What a person reads. Falls back to a humanized key. Either one string or a
    /// locale map keyed by locale (e.g. {en, de}) — curated copy carries the
    /// map, a value falling back to its own key carries the string.
    #[serde(rename = "title", default)]
    pub title: String,
    /// Semantic badge colour. The client owns what each tone looks like.
    #[serde(rename = "tone", default)]
    pub tone: String,
}
