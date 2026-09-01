use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthRecoveryConfirmRequest {
    /// The new password. It replaces the old one immediately; existing sessions
    /// are the identity service's business, not this app's.
    #[serde(rename = "password", default)]
    pub password: String,
    /// The one-time secret from the mailed link. Only that value works — it is
    /// spent on first use and expires, and anything else is a 401, so no example
    /// here would be anything but a call that fails.
    #[serde(rename = "secret", default)]
    pub secret: String,
    /// The `userId` the mailed link carried.
    #[serde(rename = "user_id", default)]
    pub user_id: String,
}
