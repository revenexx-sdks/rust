use serde::{Deserialize, Serialize};

/// Users List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserList {
    /// Total number of users that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
    /// List of users.
    #[serde(rename = "users", default)]
    pub users: Vec<crate::models::User>,
}
