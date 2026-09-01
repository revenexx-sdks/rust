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
    /// Document permissions. Each entry is a permission string: an action wrapping
    /// a role, e.g. `read("any")`, `update("user:abc")`,
    /// `delete("team:abc/owner")`. Actions are `read`, `create`, `update`,
    /// `delete` and the aggregate `write` (= create + update + delete); the role
    /// inside the quotes takes the form described under “Role strings” in this
    /// document's introduction.
    #[serde(rename = "$permissions", default)]
    pub permissions: Vec<String>,
    /// Document automatically incrementing ID.
    #[serde(rename = "$sequence", default)]
    pub sequence: i64,
    /// Document update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
}
