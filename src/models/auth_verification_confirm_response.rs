use serde::{Deserialize, Serialize};

/// The identity service's answer, forwarded verbatim: the spent verification
/// token.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthVerificationConfirmResponse {
    /// The verification that was confirmed.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// The platform user whose address is now confirmed.
    #[serde(rename = "userId", default)]
    pub user_id: String,
}
