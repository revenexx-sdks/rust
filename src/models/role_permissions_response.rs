use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RolePermissionsResponse {
    /// The role that was written.
    #[serde(rename = "key", default)]
    pub key: String,
    /// Its complete new set, after de-duplication.
    #[serde(rename = "permissions", default)]
    pub permissions: Vec<String>,
}
