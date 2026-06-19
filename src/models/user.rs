use serde::{Deserialize, Serialize};

/// User
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
    /// User creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// User ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// User update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Most recent access date in ISO 8601 format. This attribute is only updated
    /// again after 24 hours.
    #[serde(rename = "accessedAt", default)]
    pub accessed_at: String,
    /// User email address.
    #[serde(rename = "email", default)]
    pub email: String,
    /// Email verification status.
    #[serde(rename = "emailVerification", default)]
    pub email_verification: bool,
    /// Password hashing algorithm.
    #[serde(rename = "hash", default)]
    pub hash: String,
    /// Password hashing algorithm configuration.
    #[serde(rename = "hashOptions", default)]
    pub hash_options: serde_json::Value,
    /// Labels for the user.
    #[serde(rename = "labels", default)]
    pub labels: Vec<String>,
    /// Multi factor authentication status.
    #[serde(rename = "mfa", default)]
    pub mfa: bool,
    /// User name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Hashed user password.
    #[serde(rename = "password", default)]
    pub password: String,
    /// Password update time in ISO 8601 format.
    #[serde(rename = "passwordUpdate", default)]
    pub password_update: String,
    /// User phone number in E.164 format.
    #[serde(rename = "phone", default)]
    pub phone: String,
    /// Phone verification status.
    #[serde(rename = "phoneVerification", default)]
    pub phone_verification: bool,
    /// User preferences as a key-value object
    #[serde(rename = "prefs", default)]
    pub prefs: crate::models::Preferences,
    /// User registration date in ISO 8601 format.
    #[serde(rename = "registration", default)]
    pub registration: String,
    /// User status. Pass `true` for enabled and `false` for disabled.
    #[serde(rename = "status", default)]
    pub status: bool,
    /// A user-owned message receiver. A single user may have multiple e.g. emails,
    /// phones, and a browser. Each target is registered with a single provider.
    #[serde(rename = "targets", default)]
    pub targets: Vec<crate::models::Target>,
}
