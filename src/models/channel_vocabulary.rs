use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelVocabulary {
    /// The app that owns this vocabulary.
    #[serde(rename = "app", default)]
    pub app: String,
    /// Always true: the set is exhaustive at this moment, so a value outside it is
    /// stale data rather than a missing label. For a table-backed vocabulary that
    /// is a statement about now, not forever — the tenant may add to it.
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
    /// Who owns the value set. 'schema' = a CHECK constraint in this app's own
    /// schema.json; 'table' = the tenant's own rows.
    #[serde(rename = "source", default)]
    pub source: String,
    /// A plain string, or a locale map keyed by language tag ({"en": …, "de":
    /// …}). Read the requested tag, fall back to `en`.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
    /// Every permitted value, in author order — the order a select should offer,
    /// not alphabetical. For a CHECK-backed vocabulary that is the constraint's
    /// own order; for the table-backed `types` it is the tenant's `position`
    /// order.
    #[serde(rename = "values", default)]
    pub values: Vec<crate::models::ChannelVocabularyValue>,
}
