use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartVocabularyIndex {
    /// The app that owns this vocabulary.
    #[serde(rename = "app", default)]
    pub app: String,
    /// Every vocabulary this app publishes, without its values — enough to build
    /// a menu, and one call per vocabulary to fill it.
    #[serde(rename = "vocabularies", default)]
    pub vocabularies: Vec<crate::models::CartVocabularyRef>,
}
