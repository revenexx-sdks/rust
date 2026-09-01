use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormsVocabulary {
    /// The app that owns this vocabulary.
    #[serde(rename = "app", default)]
    pub app: String,
    /// The set is exhaustive.
    #[serde(rename = "closed", default)]
    pub closed: bool,
    /// The tone a value nobody gave one falls back to — what a badge looks like
    /// for a status that was added to the CHECK constraint before anyone styled
    /// it.
    #[serde(rename = "default_tone", default)]
    pub default_tone: String,
    /// A plain string, or a locale map keyed by language tag ({"en": …, "de":
    /// …}). Read the requested tag, fall back to `en`.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// Vocabulary name, unique within the app.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Parsed from the CHECK constraint.
    #[serde(rename = "source", default)]
    pub source: String,
    /// A plain string, or a locale map keyed by language tag ({"en": …, "de":
    /// …}). Read the requested tag, fall back to `en`.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
    /// Every permitted value, in constraint order — which is the order a select
    /// should offer them in, because it is the lifecycle order.
    #[serde(rename = "values", default)]
    pub values: Vec<crate::models::FormsVocabularyValue>,
}
