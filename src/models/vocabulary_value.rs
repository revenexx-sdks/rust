use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VocabularyValue {
    /// A plain string, or a locale map keyed by language tag ({ "en": …, "de":
    /// … }). Read the requested tag, fall back to `en`.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// A terminal state — nothing moves out of it. False or absent on a
    /// vocabulary that is not a lifecycle.
    #[serde(rename = "final", default)]
    pub xfinal: bool,
    /// The value as it is STORED and as the CHECK admits it — what a filter or a
    /// write sends.
    #[serde(rename = "key", default)]
    pub key: String,
    /// A plain string, or a locale map keyed by language tag ({ "en": …, "de":
    /// … }). Read the requested tag, fall back to `en`.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
    /// Which badge colour a UI should paint this value in.
    #[serde(rename = "tone", default)]
    pub tone: String,
}
