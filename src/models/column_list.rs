use serde::{Deserialize, Serialize};

/// Columns List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ColumnList {
    /// List of columns.
    #[serde(rename = "columns", default)]
    pub columns: Vec<serde_json::Value>,
    /// Total number of columns in the given table.
    #[serde(rename = "total", default)]
    pub total: i64,
}
