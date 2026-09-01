use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderListVocabularyIndex {
    /// The app that owns this vocabulary.
    #[serde(rename = "app", default)]
    pub app: String,
    /// Every vocabulary this app publishes, without its values — the values are
    /// one call further down, at GET /orderlists/vocabularies/{name}.
    #[serde(rename = "vocabularies", default)]
    pub vocabularies: Vec<serde_json::Value>,
}
