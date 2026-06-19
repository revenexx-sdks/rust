use serde::{Deserialize, Serialize};

/// Index
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ColumnIndex {
    /// Index creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Index ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Index update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Index columns.
    #[serde(rename = "columns", default)]
    pub columns: Vec<String>,
    /// Error message. Displays error generated on failure of creating or deleting
    /// an index.
    #[serde(rename = "error", default)]
    pub error: String,
    /// Index Key.
    #[serde(rename = "key", default)]
    pub key: String,
    /// Index columns length.
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
