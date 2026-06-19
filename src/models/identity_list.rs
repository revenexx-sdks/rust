use serde::{Deserialize, Serialize};

/// Identities List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdentityList {
    /// List of identities.
    #[serde(rename = "identities", default)]
    pub identities: Vec<crate::models::Identity>,
    /// Total number of identities that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
