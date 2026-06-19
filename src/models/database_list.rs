use serde::{Deserialize, Serialize};

/// Databases List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatabaseList {
    /// List of databases.
    #[serde(rename = "databases", default)]
    pub databases: Vec<crate::models::Database>,
    /// Total number of databases that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
