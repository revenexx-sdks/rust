use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartClaimRequest {
    /// Contact taking ownership.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// Guest session whose active carts are handed over.
    #[serde(rename = "session_key", default)]
    pub session_key: String,
    /// Merge the session carts into this cart instead of adopting them.
    #[serde(rename = "target_cart_id", default)]
    pub target_cart_id: String,
}
