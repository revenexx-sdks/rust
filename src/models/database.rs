use serde::{Deserialize, Serialize};

/// Database
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Database {
    /// Database creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Database ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Database update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// If database is enabled. Can be 'enabled' or 'disabled'. When disabled, the
    /// database is inaccessible to users, but remains accessible to Server SDKs
    /// using API keys.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// Database name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Database type.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
