use serde::{Deserialize, Serialize};

/// Language
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Language {
    /// Language two-character ISO 639-1 codes.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Language name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Language native name.
    #[serde(rename = "nativeName", default)]
    pub native_name: String,
}
