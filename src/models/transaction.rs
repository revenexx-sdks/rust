use serde::{Deserialize, Serialize};

/// Transaction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction creation time in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Transaction ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Transaction update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Expiration time in ISO 8601 format.
    #[serde(rename = "expiresAt", default)]
    pub expires_at: String,
    /// Number of operations in the transaction.
    #[serde(rename = "operations", default)]
    pub operations: i64,
    /// Current status of the transaction. One of: pending, committing, committed,
    /// rolled_back, failed.
    #[serde(rename = "status", default)]
    pub status: String,
}
