use serde::{Deserialize, Serialize};

/// Column Indexes List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ColumnIndexList {
    /// List of indexes.
    #[serde(rename = "indexes", default)]
    pub indexes: Vec<crate::models::ColumnIndex>,
    /// Total number of indexes that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
