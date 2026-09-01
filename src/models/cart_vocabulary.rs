use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartVocabulary {
    /// The app that owns this vocabulary.
    #[serde(rename = "app", default)]
    pub app: String,
    /// Always true here: the values come from a CHECK constraint, so the list is
    /// exhaustive and a value outside it is stale data rather than a missing
    /// label.
    #[serde(rename = "closed", default)]
    pub closed: bool,
    /// The tone a value that carries none falls back to.
    #[serde(rename = "default_tone", default)]
    pub default_tone: String,
    /// A plain string, or a locale map keyed by language tag ({"en": …, "de":
    /// …}). Read the requested tag, fall back to `en`.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// Vocabulary name, unique within the app.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Where the values came from. 'schema' = a CHECK constraint in this app's own
    /// schema.json.
    #[serde(rename = "source", default)]
    pub source: String,
    /// A plain string, or a locale map keyed by language tag ({"en": …, "de":
    /// …}). Read the requested tag, fall back to `en`.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
    /// Every permitted value, in the order the CHECK constraint lists them —
    /// which is the order a select should offer them in.
    #[serde(rename = "values", default)]
    pub values: Vec<crate::models::CartVocabularyValue>,
}
