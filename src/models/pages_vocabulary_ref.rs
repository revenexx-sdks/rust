use serde::{Deserialize, Serialize};

/// One vocabulary, named but not unpacked.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PagesVocabularyRef {
    /// What the set is for, or null. A plain string, or a locale map keyed by
    /// language tag ({ "en": …, "de": … }). Read the requested tag, fall back
    /// to `en`.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// The name to fetch it by — the part after the dot in the qualified id.
    #[serde(rename = "name", default)]
    pub name: String,
    /// What this set of values is called. A plain string, or a locale map keyed by
    /// language tag ({ "en": …, "de": … }). Read the requested tag, fall back
    /// to `en`.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
}
