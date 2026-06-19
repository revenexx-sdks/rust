use serde::{Deserialize, Serialize};

/// Resource Tokens List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceTokenList {
    /// List of tokens.
    #[serde(rename = "tokens", default)]
    pub tokens: Vec<crate::models::ResourceToken>,
    /// Total number of tokens that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
