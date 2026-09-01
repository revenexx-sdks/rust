use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthMfaChallengeConfirmRequest {
    /// The `$id` the send answered with.
    #[serde(rename = "challenge_id", default)]
    pub challenge_id: String,
    /// What the buyer typed.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The same session the challenge was created with.
    #[serde(rename = "session_secret", default)]
    pub session_secret: String,
    /// The platform user, for the caller's own bookkeeping. The challenge already
    /// knows whose it is.
    #[serde(rename = "user_id", default)]
    pub user_id: String,
}
