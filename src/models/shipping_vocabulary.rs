use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingVocabulary {
    /// The app that owns this vocabulary.
    #[serde(rename = "app", default)]
    pub app: String,
    /// The set is exhaustive, so a value outside it is stale data rather than a
    /// missing label. True either way — what differs is who may extend it.
    #[serde(rename = "closed", default)]
    pub closed: bool,
    /// The badge colour a value that names none falls back to.
    #[serde(rename = "default_tone", default)]
    pub default_tone: String,
    /// What the vocabulary is for. Either one string or a locale map keyed by
    /// locale (e.g. {en, de}) — curated copy carries the map, a value falling
    /// back to its own key carries the string.
    #[serde(rename = "description", default)]
    pub description: String,
    /// The vocabulary name — the part after the dot in the qualified id.
    #[serde(rename = "name", default)]
    pub name: String,
    /// 'schema' — the values are a CHECK constraint's, so the served set IS the
    /// enforced set. 'table' — the values are the tenant's own rows, read per
    /// request.
    #[serde(rename = "source", default)]
    pub source: String,
    /// What the vocabulary is called. Either one string or a locale map keyed by
    /// locale (e.g. {en, de}) — curated copy carries the map, a value falling
    /// back to its own key carries the string.
    #[serde(rename = "title", default)]
    pub title: String,
    /// Every permitted value, in the order a select should offer them —
    /// constraint order for a schema vocabulary, `position` for a table one.
    #[serde(rename = "values", default)]
    pub values: Vec<crate::models::ShippingVocabularyValue>,
}
