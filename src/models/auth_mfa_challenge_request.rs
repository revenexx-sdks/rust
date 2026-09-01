use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthMfaChallengeRequest {
    /// Which factor to challenge. Defaults to `email`, the only one this route
    /// mails.
    #[serde(rename = "factor", default)]
    pub factor: String,
    /// The platform user being challenged.
    #[serde(rename = "user_id", default)]
    pub user_id: String,
}
