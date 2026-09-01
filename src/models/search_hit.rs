use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchHit {
    /// The matching document; its properties are the collection's own fields.
    #[serde(rename = "document", default)]
    pub document: serde_json::Value,
    /// Per-field highlight snippets, keyed by field name.
    #[serde(rename = "highlight", default)]
    pub highlight: serde_json::Value,
    /// Relevance score.
    #[serde(rename = "text_match", default)]
    pub text_match: i64,
}
