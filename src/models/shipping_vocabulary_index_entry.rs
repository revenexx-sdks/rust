use serde::{Deserialize, Serialize};

/// One vocabulary, named and titled.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingVocabularyIndexEntry {
    /// What the vocabulary is for. Either one string or a locale map keyed by
    /// locale (e.g. {en, de}) — curated copy carries the map, a value falling
    /// back to its own key carries the string.
    #[serde(rename = "description", default)]
    pub description: String,
    /// The part after the dot in the qualified id — what GET
    /// /shipping/vocabularies/{name} takes.
    #[serde(rename = "name", default)]
    pub name: String,
    /// What the vocabulary is called. Either one string or a locale map keyed by
    /// locale (e.g. {en, de}) — curated copy carries the map, a value falling
    /// back to its own key carries the string.
    #[serde(rename = "title", default)]
    pub title: String,
}
