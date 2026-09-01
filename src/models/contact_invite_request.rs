use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactInviteRequest {
    /// Who did the inviting, as the recipient should read it. Absent, the company
    /// name is used — "Beispiel GmbH invited you" reads better than the name of
    /// somebody they have never heard of.
    #[serde(rename = "invited_by", default)]
    pub invited_by: String,
    /// Where the invitation points — the storefront sign-in, normally. There is
    /// no token in it: the person is already a member and only has to sign in.
    #[serde(rename = "url", default)]
    pub url: String,
}
