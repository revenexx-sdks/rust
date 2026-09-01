use serde::{Deserialize, Serialize};

/// The token, minus its secret. The secret travels only in the mailed link —
/// a caller holding both would not need the mail at all.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthMagicLinkResponse {
    /// The token that was created.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// When the link stops working. The mail says the same in words.
    #[serde(rename = "expire", default)]
    pub expire: String,
    /// Which template the buyer received: 'tenant' is this shop's own, 'platform'
    /// the identity service's built-in one — the fallback when messaging could
    /// not be reached. The value is the same either way, so the flow works in both
    /// cases.
    #[serde(rename = "mail", default)]
    pub mail: String,
    /// The platform user it belongs to — new when the address was.
    #[serde(rename = "userId", default)]
    pub user_id: String,
}
