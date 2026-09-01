use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VocabularyRef {
    /// A plain string, or a locale map keyed by language tag ({ "en": …, "de":
    /// … }). Read the requested tag, fall back to `en`.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// The name to pass to `GET /products/vocabularies/{name}`.
    #[serde(rename = "name", default)]
    pub name: String,
    /// A plain string, or a locale map keyed by language tag ({ "en": …, "de":
    /// … }). Read the requested tag, fall back to `en`.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
}
