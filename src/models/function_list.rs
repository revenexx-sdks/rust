use serde::{Deserialize, Serialize};

/// Functions List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FunctionList {
    /// List of functions.
    #[serde(rename = "functions", default)]
    pub functions: Vec<crate::models::Function>,
    /// Total number of functions that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
