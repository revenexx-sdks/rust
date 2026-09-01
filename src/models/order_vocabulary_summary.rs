use serde::{Deserialize, Serialize};

/// One vocabulary, named and titled but without its values.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderVocabularySummary {
    /// Either one string, or a map of locale to string ({"en": …, "de": …}).
    #[serde(rename = "description", default)]
    pub description: String,
    /// Vocabulary name, unique within the app.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Either one string, or a map of locale to string ({"en": …, "de": …}).
    #[serde(rename = "title", default)]
    pub title: String,
}
