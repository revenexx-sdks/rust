use serde::{Deserialize, Serialize};

/// One permitted value of a vocabulary, with everything needed to render it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PagesVocabularyValue {
    /// When to use this value, or null when nobody wrote one. A plain string, or a
    /// locale map keyed by language tag ({ "en": …, "de": … }). Read the
    /// requested tag, fall back to `en`.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// The value ends the lifecycle.
    #[serde(rename = "final", default)]
    pub xfinal: bool,
    /// The value as the database stores and enforces it.
    #[serde(rename = "key", default)]
    pub key: String,
    /// What a person reads. Falls back to a humanized key. A plain string, or a
    /// locale map keyed by language tag ({ "en": …, "de": … }). Read the
    /// requested tag, fall back to `en`.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
    /// Semantic badge colour. The client owns what each tone looks like.
    #[serde(rename = "tone", default)]
    pub tone: String,
}
