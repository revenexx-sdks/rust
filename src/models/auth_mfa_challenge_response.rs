use serde::{Deserialize, Serialize};

/// The challenge, minus the code. The code is in the mail; a storefront that
/// also received it would not be asking for a second factor.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthMfaChallengeResponse {
    /// The challenge — send it back as `challenge_id` with the code the buyer
    /// types.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// When the code stops working.
    #[serde(rename = "expire", default)]
    pub expire: String,
    /// Which template the buyer received: 'tenant' is this shop's own, 'platform'
    /// the identity service's built-in one — the fallback when messaging could
    /// not be reached. The value is the same either way, so the flow works in both
    /// cases.
    #[serde(rename = "mail", default)]
    pub mail: String,
    /// The platform user it belongs to.
    #[serde(rename = "userId", default)]
    pub user_id: String,
}
