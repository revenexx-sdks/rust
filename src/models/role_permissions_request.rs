use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RolePermissionsRequest {
    /// The complete new set. Duplicates and blanks are ignored; an empty array
    /// revokes everything.
    #[serde(rename = "permissions", default)]
    pub permissions: Vec<String>,
}
