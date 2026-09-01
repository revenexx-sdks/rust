use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VocabularyIndex {
    /// This app's name — the part before the dot in the qualified id
    /// `customers.<name>`.
    #[serde(rename = "app", default)]
    pub app: String,
    /// Every vocabulary this app publishes, without their values.
    #[serde(rename = "vocabularies", default)]
    pub vocabularies: Vec<serde_json::Value>,
}
