use serde::{Deserialize, Serialize};

/// Rows List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RowList {
    /// List of rows.
    #[serde(rename = "rows", default)]
    pub rows: Vec<crate::models::Row>,
    /// Total number of rows that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
