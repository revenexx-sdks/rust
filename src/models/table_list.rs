use serde::{Deserialize, Serialize};

/// Tables List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableList {
    /// List of tables.
    #[serde(rename = "tables", default)]
    pub tables: Vec<crate::models::Table>,
    /// Total number of tables that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
