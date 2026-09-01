use serde::{Deserialize, Serialize};

/// Collection
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Collection2 {
    /// Collection creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Collection ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Collection permissions. Each entry is a permission string: an action
    /// wrapping a role, e.g. `read("any")`, `update("user:abc")`,
    /// `delete("team:abc/owner")`. Actions are `read`, `create`, `update`,
    /// `delete` and the aggregate `write` (= create + update + delete); the role
    /// inside the quotes takes the form described under “Role strings” in this
    /// document's introduction.
    #[serde(rename = "$permissions", default)]
    pub permissions: Vec<String>,
    /// Collection update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Collection attributes.
    #[serde(rename = "attributes", default)]
    pub attributes: Vec<serde_json::Value>,
    /// Maximum document size in bytes. Returns 0 when no limit applies.
    #[serde(rename = "bytesMax", default)]
    pub bytes_max: i64,
    /// Currently used document size in bytes based on defined attributes.
    #[serde(rename = "bytesUsed", default)]
    pub bytes_used: i64,
    /// Database ID.
    #[serde(rename = "databaseId", default)]
    pub database_id: String,
    /// Whether document-level permissions are enabled. When it is, each record's
    /// own `$permissions` are enforced on top of the container's.
    #[serde(rename = "documentSecurity", default)]
    pub document_security: bool,
    /// Collection enabled. Can be 'enabled' or 'disabled'. When disabled, the
    /// collection is inaccessible to users, but remains accessible to Server SDKs
    /// using API keys.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// Collection indexes.
    #[serde(rename = "indexes", default)]
    pub indexes: Vec<crate::models::Index>,
    /// Collection name.
    #[serde(rename = "name", default)]
    pub name: String,
}
