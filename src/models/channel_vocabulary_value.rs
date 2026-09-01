use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelVocabularyValue {
    /// A plain string, or a locale map keyed by language tag ({"en": …, "de":
    /// …}). Read the requested tag, fall back to `en`.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// Table-backed vocabularies only: the localized descriptions. A locale map
    /// keyed by language tag: {"en": …, "de": …}. Read the requested tag and
    /// fall back to the plain column beside it.
    #[serde(rename = "descriptions", default)]
    pub descriptions: serde_json::Value,
    /// The value ends the lifecycle.
    #[serde(rename = "final", default)]
    pub xfinal: bool,
    /// Table-backed vocabularies only: the value a create falls back to.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Table-backed vocabularies only: seeded on install rather than added by the
    /// tenant. Still renameable and retirable.
    #[serde(rename = "is_system", default)]
    pub is_system: bool,
    /// The value as the database stores and enforces it.
    #[serde(rename = "key", default)]
    pub key: String,
    /// Table-backed vocabularies only: the localized titles. `title` stays the
    /// fallback. A locale map keyed by language tag: {"en": …, "de": …}. Read
    /// the requested tag and fall back to the plain column beside it.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// A plain string, or a locale map keyed by language tag ({"en": …, "de":
    /// …}). Read the requested tag, fall back to `en`.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
    /// Semantic badge colour. The client owns what each tone looks like.
    #[serde(rename = "tone", default)]
    pub tone: String,
}
