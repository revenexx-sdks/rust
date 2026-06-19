use serde::{Deserialize, Serialize};

/// Document
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Document {
    /// Collection ID.
    #[serde(rename = "$collectionId", default)]
    pub collection_id: String,
    /// Document creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Database ID.
    #[serde(rename = "$databaseId", default)]
    pub database_id: String,
    /// Document ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Document permissions. [Learn more about
    /// permissions](https://appwrite.io/docs/permissions).
    #[serde(rename = "$permissions", default)]
    pub permissions: Vec<String>,
    /// Document automatically incrementing ID.
    #[serde(rename = "$sequence", default)]
    pub sequence: i64,
    /// Document update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
}
