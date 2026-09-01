use serde::{Deserialize, Serialize};

/// One permitted value, with the words and the colour a human reads for it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentVocabularyValue {
    /// One sentence on what the value means, or null where the key speaks for
    /// itself. A plain string, or a locale map keyed by language tag ({ "en": …,
    /// "de": … }). Read the requested tag, fall back to `en`.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// This value ends the lifecycle — the honest way to ask "is this still
    /// open?" instead of matching status names.
    #[serde(rename = "final", default)]
    pub xfinal: bool,
    /// The value exactly as the database stores it — what a filter sends and
    /// what a row carries.
    #[serde(rename = "key", default)]
    pub key: String,
    /// The label to show for this value. A plain string, or a locale map keyed by
    /// language tag ({ "en": …, "de": … }). Read the requested tag, fall back
    /// to `en`.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
    /// What the state MEANS, semantically: neutral, info, success, warning or
    /// danger. The client decides what each one looks like in its own design
    /// system.
    #[serde(rename = "tone", default)]
    pub tone: String,
}
