use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderListVocabulary {
    /// The app that owns this vocabulary.
    #[serde(rename = "app", default)]
    pub app: String,
    /// The set is exhaustive: a value outside it is stale data, not a missing
    /// label.
    #[serde(rename = "closed", default)]
    pub closed: bool,
    /// The badge colour a value carries when it names none of its own.
    #[serde(rename = "default_tone", default)]
    pub default_tone: String,
    /// A plain string, or a locale map keyed by language tag ({"en": …, "de":
    /// …}). Read the requested tag, fall back to `en`.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// Vocabulary name, unique within the app.
    #[serde(rename = "name", default)]
    pub name: String,
    /// 'schema' — a CHECK constraint owns the set; 'table' — the tenant's own
    /// rows do.
    #[serde(rename = "source", default)]
    pub source: String,
    /// A plain string, or a locale map keyed by language tag ({"en": …, "de":
    /// …}). Read the requested tag, fall back to `en`.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
    /// Every permitted value, in the order a select should offer them.
    #[serde(rename = "values", default)]
    pub values: Vec<crate::models::OrderListVocabularyValue>,
}
