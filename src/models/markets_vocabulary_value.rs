use serde::{Deserialize, Serialize};

/// One permitted value, with the copy and the badge tone a client renders it
/// as.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketsVocabularyValue {
    /// Either one string, or a map of locale to string ({"en": …, "de": …}).
    #[serde(rename = "description", default)]
    pub description: String,
    /// A terminal state nothing moves out of.
    #[serde(rename = "final", default)]
    pub xfinal: bool,
    /// The value as stored in the column.
    #[serde(rename = "key", default)]
    pub key: String,
    /// Either one string, or a map of locale to string ({"en": …, "de": …}).
    #[serde(rename = "title", default)]
    pub title: String,
    /// Semantic badge tone — the client decides what it looks like.
    #[serde(rename = "tone", default)]
    pub tone: String,
}
