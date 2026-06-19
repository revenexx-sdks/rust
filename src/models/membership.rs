use serde::{Deserialize, Serialize};

/// Membership
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Membership {
    /// Membership creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Membership ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Membership update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// User confirmation status, true if the user has joined the team or false
    /// otherwise.
    #[serde(rename = "confirm", default)]
    pub confirm: bool,
    /// Date, the user has been invited to join the team in ISO 8601 format.
    #[serde(rename = "invited", default)]
    pub invited: String,
    /// Date, the user has accepted the invitation to join the team in ISO 8601
    /// format.
    #[serde(rename = "joined", default)]
    pub joined: String,
    /// Multi factor authentication status, true if the user has MFA enabled or
    /// false otherwise. Hide this attribute by toggling membership privacy in the
    /// Console.
    #[serde(rename = "mfa", default)]
    pub mfa: bool,
    /// User list of roles
    #[serde(rename = "roles", default)]
    pub roles: Vec<String>,
    /// Team ID.
    #[serde(rename = "teamId", default)]
    pub team_id: String,
    /// Team name.
    #[serde(rename = "teamName", default)]
    pub team_name: String,
    /// User email address. Hide this attribute by toggling membership privacy in
    /// the Console.
    #[serde(rename = "userEmail", default)]
    pub user_email: String,
    /// User ID.
    #[serde(rename = "userId", default)]
    pub user_id: String,
    /// User name. Hide this attribute by toggling membership privacy in the
    /// Console.
    #[serde(rename = "userName", default)]
    pub user_name: String,
}
