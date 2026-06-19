use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthMeRequest {
    /// Optional session to verify — answers 401 when the session is expired or
    /// revoked.
    #[serde(rename = "session_id", default)]
    pub session_id: String,
    #[serde(rename = "user_id", default)]
    pub user_id: String,
}
