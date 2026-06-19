use serde::{Deserialize, Serialize};

/// Platform auth session. Treat `secret` as a credential — the trusted BFF
/// stores it server-side (HTTP-only cookie), never in the browser.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthSession {
    #[serde(rename = "$id", default)]
    pub id: String,
    #[serde(rename = "expire", default)]
    pub expire: String,
    #[serde(rename = "provider", default)]
    pub provider: String,
    #[serde(rename = "secret", default)]
    pub secret: String,
    #[serde(rename = "userId", default)]
    pub user_id: String,
}
