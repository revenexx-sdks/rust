use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactInviteResponse {
    /// Who was invited.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// Always true when this answers — a failure to send is a 502, not a false
    /// here.
    #[serde(rename = "invited", default)]
    pub invited: bool,
    /// The company they were invited into.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
}
