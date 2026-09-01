use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderVocabularyIndex {
    /// This app's name — the part before the dot in the qualified id.
    #[serde(rename = "app", default)]
    pub app: String,
    /// Every vocabulary this app publishes, without its values — fetch one with
    /// GET /orders/vocabularies/{name}.
    #[serde(rename = "vocabularies", default)]
    pub vocabularies: Vec<crate::models::OrderVocabularySummary>,
}
