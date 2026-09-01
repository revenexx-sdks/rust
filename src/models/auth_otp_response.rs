use serde::{Deserialize, Serialize};

/// The token, minus the code. The code is in the mail and nowhere else.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthOtpResponse {
    /// The token that was created.
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
    /// The platform user it belongs to — send it back with the code the buyer
    /// types.
    #[serde(rename = "userId", default)]
    pub user_id: String,
}
