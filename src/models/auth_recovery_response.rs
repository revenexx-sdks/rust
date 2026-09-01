use serde::{Deserialize, Serialize};

/// The identity service's recovery token, minus its secret, plus which mail
/// the customer got. The secret is stripped deliberately — it travels only
/// in the mailed link, and a caller that had both would not need the mail at
/// all. `mail` is `tenant` when this shop's own template went out and
/// `platform` when the messaging service could not be reached and the identity
/// service's built-in mail is the copy the buyer has; the link is the same
/// either way.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthRecoveryResponse {
    /// The recovery that was created.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// When the link stops working. The mail says the same thing in words.
    #[serde(rename = "expire", default)]
    pub expire: String,
    /// Which template the buyer received: 'tenant' is this shop's own, 'platform'
    /// the identity service's built-in one — the fallback when messaging could
    /// not be reached. The link is identical either way, so a reset works in both
    /// cases.
    #[serde(rename = "mail", default)]
    pub mail: String,
    /// The platform user it belongs to.
    #[serde(rename = "userId", default)]
    pub user_id: String,
}
