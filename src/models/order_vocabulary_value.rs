use serde::{Deserialize, Serialize};

/// One permitted value with the words and the badge tone a client should
/// render for it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderVocabularyValue {
    /// Either one string, or a map of locale to string ({"en": …, "de": …}).
    #[serde(rename = "description", default)]
    pub description: String,
    /// True when this value ENDS the lifecycle. Lets a reader ask "is this order
    /// still open?" instead of matching status names it guessed.
    #[serde(rename = "final", default)]
    pub xfinal: bool,
    /// The value as stored — exactly what the CHECK constraint permits.
    #[serde(rename = "key", default)]
    pub key: String,
    /// Only on 'return-resolutions': which return transition accepts this value. A
    /// settlement word on the refusal dialog is how the two sets got mixed up.
    #[serde(rename = "stage", default)]
    pub stage: String,
    /// Either one string, or a map of locale to string ({"en": …, "de": …}).
    #[serde(rename = "title", default)]
    pub title: String,
    /// Semantic badge colour. The client owns what each tone looks like.
    #[serde(rename = "tone", default)]
    pub tone: String,
}
