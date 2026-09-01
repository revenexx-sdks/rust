use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthVerificationRequest {
    /// Where the mailed link points. `userId`, `secret` and `expire` are appended
    /// as query parameters; the first two are what the confirm call takes.
    #[serde(rename = "url", default)]
    pub url: String,
    /// The platform user whose address is being confirmed — `user_id` from the
    /// registration, or `session.userId` from a login.
    #[serde(rename = "user_id", default)]
    pub user_id: String,
}
