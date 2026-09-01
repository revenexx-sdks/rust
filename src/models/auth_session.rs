use serde::{Deserialize, Serialize};

/// Platform auth session. Treat `secret` as a credential — the trusted BFF
/// stores it server-side (HTTP-only cookie), never in the browser.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthSession {
    /// The session id. Send it back as `session_id` to log out, or to have
    /// `/auth/me` check that the session is still alive.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// When the session stops being valid on its own.
    #[serde(rename = "expire", default)]
    pub expire: String,
    /// How the session was created. Server-minted sessions from this route are not
    /// the browser-facing email/password ones, so this says which mechanism issued
    /// it.
    #[serde(rename = "provider", default)]
    pub provider: String,
    /// The session CREDENTIAL. Whoever holds it is logged in — the BFF keeps it
    /// server-side (an HTTP-only cookie), never in the browser and never in a log.
    #[serde(rename = "secret", default)]
    pub secret: String,
    /// The platform user this session belongs to — the `user_id` every other
    /// auth route takes. NOT the contact id: the contact is in `contact`.
    #[serde(rename = "userId", default)]
    pub user_id: String,
}
