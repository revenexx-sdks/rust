use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrincipalResolveRequest {
    /// The contact the caller is acting for.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
}
