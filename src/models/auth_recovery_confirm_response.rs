use serde::{Deserialize, Serialize};

/// The identity service's answer, forwarded verbatim: the spent recovery
/// token. The new password is already in effect when this arrives.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthRecoveryConfirmResponse {
    /// The recovery that was confirmed.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// The platform user whose password was set.
    #[serde(rename = "userId", default)]
    pub user_id: String,
}
