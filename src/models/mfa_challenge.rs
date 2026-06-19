use serde::{Deserialize, Serialize};

/// MFA Challenge
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MfaChallenge {
    /// Token creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Token ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Token expiration date in ISO 8601 format.
    #[serde(rename = "expire", default)]
    pub expire: String,
    /// User ID.
    #[serde(rename = "userId", default)]
    pub user_id: String,
}
