use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthMagicLinkRequest {
    /// Who to send the link to. An address that has never been seen creates an
    /// account rather than failing.
    #[serde(rename = "email", default)]
    pub email: String,
    /// Where the mailed link points. `userId`, `secret` and `expire` are appended
    /// as query parameters; the first two are what the confirm call takes.
    #[serde(rename = "url", default)]
    pub url: String,
}
