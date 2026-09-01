use serde::{Deserialize, Serialize};

/// One closed value set this app owns, parsed out of the CHECK constraint in
/// schema.json — the served set IS the enforced set. `closed: true` means a
/// client may treat anything outside `values` as stale data.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketsVocabulary {
    /// The app that owns this vocabulary.
    #[serde(rename = "app", default)]
    pub app: String,
    /// Always true here: the values come from a CHECK constraint, so the list is
    /// exhaustive.
    #[serde(rename = "closed", default)]
    pub closed: bool,
    /// The tone a value that carries none falls back to.
    #[serde(rename = "default_tone", default)]
    pub default_tone: String,
    /// Either one string, or a map of locale to string ({"en": …, "de": …}).
    #[serde(rename = "description", default)]
    pub description: String,
    /// Vocabulary name, unique within the app.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Where the values came from. 'schema' = a CHECK constraint in this app's own
    /// schema.json.
    #[serde(rename = "source", default)]
    pub source: String,
    /// Either one string, or a map of locale to string ({"en": …, "de": …}).
    #[serde(rename = "title", default)]
    pub title: String,
    /// Every value the column may hold, in the order the CHECK constraint lists
    /// them — which is the order a select box should offer them in. Exhaustive,
    /// because `closed` is true.
    #[serde(rename = "values", default)]
    pub values: Vec<crate::models::MarketsVocabularyValue>,
}
