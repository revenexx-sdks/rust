use serde::{Deserialize, Serialize};

/// Every closed value set this app owns, by name — enough to build a menu of
/// them without fetching each one.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketsVocabularyIndex {
    /// The app that owns this vocabulary.
    #[serde(rename = "app", default)]
    pub app: String,
    /// Every vocabulary this app publishes, named and titled but without its
    /// values — fetch one by name for those.
    #[serde(rename = "vocabularies", default)]
    pub vocabularies: Vec<crate::models::MarketsVocabularySummary>,
}
