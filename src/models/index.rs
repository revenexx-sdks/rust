use serde::{Deserialize, Serialize};

/// Index
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Index {
    /// Index creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Index ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Index update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Index attributes.
    #[serde(rename = "attributes", default)]
    pub attributes: Vec<String>,
    /// Error message. Displays error generated on failure of creating or deleting
    /// an index.
    #[serde(rename = "error", default)]
    pub error: String,
    /// Index key.
    #[serde(rename = "key", default)]
    pub key: String,
    /// Index attributes length.
    #[serde(rename = "lengths", default)]
    pub lengths: Vec<i64>,
    /// Index orders.
    #[serde(rename = "orders", default)]
    pub orders: Vec<String>,
    /// Index status. Possible values: `available`, `processing`, `deleting`,
    /// `stuck`, or `failed`
    #[serde(rename = "status", default)]
    pub status: String,
    /// Index type.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
