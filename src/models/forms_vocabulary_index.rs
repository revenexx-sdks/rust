use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormsVocabularyIndex {
    /// The app that owns this vocabulary.
    #[serde(rename = "app", default)]
    pub app: String,
    /// Every vocabulary this app publishes, without its values — enough to build
    /// a menu, not enough to fill a select. Fetch one by name for that.
    #[serde(rename = "vocabularies", default)]
    pub vocabularies: Vec<crate::models::FormsVocabularySummary>,
}
