use serde::{Deserialize, Serialize};

/// Identity
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Identity {
    /// Identity creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Identity ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Identity update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Identity Provider.
    #[serde(rename = "provider", default)]
    pub provider: String,
    /// Identity Provider Access Token.
    #[serde(rename = "providerAccessToken", default)]
    pub provider_access_token: String,
    /// The date of when the access token expires in ISO 8601 format.
    #[serde(rename = "providerAccessTokenExpiry", default)]
    pub provider_access_token_expiry: String,
    /// Email of the User in the Identity Provider.
    #[serde(rename = "providerEmail", default)]
    pub provider_email: String,
    /// Identity Provider Refresh Token.
    #[serde(rename = "providerRefreshToken", default)]
    pub provider_refresh_token: String,
    /// ID of the User in the Identity Provider.
    #[serde(rename = "providerUid", default)]
    pub provider_uid: String,
    /// User ID.
    #[serde(rename = "userId", default)]
    pub user_id: String,
}
