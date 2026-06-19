use serde::{Deserialize, Serialize};

/// Table
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Table {
    /// Table creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Table ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Table permissions. [Learn more about
    /// permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions", default)]
    pub permissions: Vec<String>,
    /// Table update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Maximum row size in bytes. Returns 0 when no limit applies.
    #[serde(rename = "bytesMax", default)]
    pub bytes_max: i64,
    /// Currently used row size in bytes based on defined columns.
    #[serde(rename = "bytesUsed", default)]
    pub bytes_used: i64,
    /// Table columns.
    #[serde(rename = "columns", default)]
    pub columns: Vec<serde_json::Value>,
    /// Database ID.
    #[serde(rename = "databaseId", default)]
    pub database_id: String,
    /// Table enabled. Can be 'enabled' or 'disabled'. When disabled, the table is
    /// inaccessible to users, but remains accessible to Server SDKs using API
    /// keys.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// Table indexes.
    #[serde(rename = "indexes", default)]
    pub indexes: Vec<crate::models::ColumnIndex>,
    /// Table name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Whether row-level permissions are enabled. [Learn more about
    /// permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "rowSecurity", default)]
    pub row_security: bool,
}
