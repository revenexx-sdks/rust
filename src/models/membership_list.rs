use serde::{Deserialize, Serialize};

/// Memberships List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MembershipList {
    /// List of memberships.
    #[serde(rename = "memberships", default)]
    pub memberships: Vec<crate::models::Membership>,
    /// Total number of memberships that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
