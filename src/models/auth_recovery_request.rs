use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthRecoveryRequest {
    #[serde(rename = "email", default)]
    pub email: String,
    /// Redirect URL carrying userId + secret.
    #[serde(rename = "url", default)]
    pub url: String,
}
