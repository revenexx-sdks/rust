use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RolesDefaultsResponse {
    /// Role keys created by this call.
    #[serde(rename = "created", default)]
    pub created: Vec<String>,
    /// Role keys that were already there and were left untouched, permissions
    /// included.
    #[serde(rename = "existing", default)]
    pub existing: Vec<String>,
}
