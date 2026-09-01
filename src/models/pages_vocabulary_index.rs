use serde::{Deserialize, Serialize};

/// Which vocabularies this app publishes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PagesVocabularyIndex {
    /// Always 'pages' — the first half of the qualified id a client holds.
    #[serde(rename = "app", default)]
    pub app: String,
    /// One entry per vocabulary, without its values.
    #[serde(rename = "vocabularies", default)]
    pub vocabularies: Vec<crate::models::PagesVocabularyRef>,
}
