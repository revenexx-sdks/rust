use serde::{Deserialize, Serialize};

/// Languages List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguageList {
    /// List of languages.
    #[serde(rename = "languages", default)]
    pub languages: Vec<crate::models::Language>,
    /// Total number of languages that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
