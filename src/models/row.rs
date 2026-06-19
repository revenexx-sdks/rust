use serde::{Deserialize, Serialize};

/// Row
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Row {
    /// Row creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Database ID.
    #[serde(rename = "$databaseId", default)]
    pub database_id: String,
    /// Row ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Row permissions. [Learn more about
    /// permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions", default)]
    pub permissions: Vec<String>,
    /// Row automatically incrementing ID.
    #[serde(rename = "$sequence", default)]
    pub sequence: i64,
    /// Table ID.
    #[serde(rename = "$tableId", default)]
    pub table_id: String,
    /// Row update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
}
