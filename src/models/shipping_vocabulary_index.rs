use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingVocabularyIndex {
    /// The app that owns these vocabularies — the part before the dot in a
    /// qualified id.
    #[serde(rename = "app", default)]
    pub app: String,
    /// Every vocabulary this app publishes, without its values. Names only: fetch
    /// one to get the set.
    #[serde(rename = "vocabularies", default)]
    pub vocabularies: Vec<crate::models::ShippingVocabularyIndexEntry>,
}
