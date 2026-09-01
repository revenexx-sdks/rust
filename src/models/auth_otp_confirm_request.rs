use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthOtpConfirmRequest {
    /// The one-time secret the mailed code carried. Spent on first use and
    /// expiring, so a second attempt with the same one is a 401 rather than a
    /// second session.
    #[serde(rename = "secret", default)]
    pub secret: String,
    /// The `userId` the mailed code carried.
    #[serde(rename = "user_id", default)]
    pub user_id: String,
}
