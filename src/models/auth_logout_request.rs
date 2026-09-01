use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthLogoutRequest {
    /// The session to revoke — `session.$id` from the login.
    #[serde(rename = "session_id", default)]
    pub session_id: String,
    /// The platform user — `session.userId` from the login.
    #[serde(rename = "user_id", default)]
    pub user_id: String,
}
