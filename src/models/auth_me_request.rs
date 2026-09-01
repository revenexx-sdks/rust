use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthMeRequest {
    /// Optional session to verify. Pass it to ask "is this session still alive?"
    /// (a revoked one is then a 401); omit it to only ask who a user is.
    #[serde(rename = "session_id", default)]
    pub session_id: String,
    /// The platform user to resolve — `session.userId` from the login.
    #[serde(rename = "user_id", default)]
    pub user_id: String,
}
