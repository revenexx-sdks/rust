use serde::{Deserialize, Serialize};

/// One vocabulary, named and titled — fetch its values with GET
/// /prices/vocabularies/{name}.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceVocabularyRef {
    /// A plain string, or a locale map keyed by language tag ({"en": …, "de":
    /// …}). Read the requested tag, fall back to `en`.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// Vocabulary name, unique within the app.
    #[serde(rename = "name", default)]
    pub name: String,
    /// A plain string, or a locale map keyed by language tag ({"en": …, "de":
    /// …}). Read the requested tag, fall back to `en`.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
}
