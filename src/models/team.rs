use serde::{Deserialize, Serialize};

/// Team
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Team {
    /// Team creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Team ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Team update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Team name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Team preferences as a key-value object
    #[serde(rename = "prefs", default)]
    pub prefs: crate::models::Preferences,
    /// Total number of team members.
    #[serde(rename = "total", default)]
    pub total: i64,
}
