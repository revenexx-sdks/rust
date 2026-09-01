use serde::{Deserialize, Serialize};

/// What this app publishes, without the values — one fetch a UI can cache
/// and then pull only the vocabularies it renders.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceVocabularyIndex {
    /// The app that owns this vocabulary.
    #[serde(rename = "app", default)]
    pub app: String,
    /// Every vocabulary this app owns, sorted by name.
    #[serde(rename = "vocabularies", default)]
    pub vocabularies: Vec<crate::models::PriceVocabularyRef>,
}
