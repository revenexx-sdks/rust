use serde::{Deserialize, Serialize};

/// The identity service's answer, forwarded verbatim.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthMfaChallengeConfirmResponse {
    /// The challenge that was answered.
    #[serde(rename = "$id", default)]
    pub id: String,
}
