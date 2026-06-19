use serde::{Deserialize, Serialize};

/// Target
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Target {
    /// Target creation time in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Target ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Target update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Is the target expired.
    #[serde(rename = "expired", default)]
    pub expired: bool,
    /// The target identifier.
    #[serde(rename = "identifier", default)]
    pub identifier: String,
    /// Target Name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Provider ID.
    #[serde(rename = "providerId", default)]
    pub provider_id: String,
    /// The target provider type. Can be one of the following: `email`, `sms` or
    /// `push`.
    #[serde(rename = "providerType", default)]
    pub provider_type: String,
    /// User ID.
    #[serde(rename = "userId", default)]
    pub user_id: String,
}
