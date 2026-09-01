use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelVocabularyIndex {
    /// The app that owns this vocabulary.
    #[serde(rename = "app", default)]
    pub app: String,
    /// Every vocabulary this app owns, alphabetically: statuses, types,
    /// unassigned-visibility. Names only — fetch the values with GET
    /// /channels/vocabularies/{name}.
    #[serde(rename = "vocabularies", default)]
    pub vocabularies: Vec<crate::models::ChannelVocabularyRef>,
}
