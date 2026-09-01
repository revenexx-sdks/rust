use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthRecoveryRequest {
    /// Who to send the recovery mail to. An address nobody holds is not
    /// distinguished here — do not build an account-existence check on the
    /// answer.
    #[serde(rename = "email", default)]
    pub email: String,
    /// Where the mailed link points. `userId`, `secret` and `expire` are appended
    /// as query parameters — the first two are what the confirm call takes. Same
    /// shape the identity service's own mail used, so a storefront that already
    /// handles that link needs no change.
    #[serde(rename = "url", default)]
    pub url: String,
}
