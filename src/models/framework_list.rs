use serde::{Deserialize, Serialize};

/// Frameworks List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FrameworkList {
    /// List of frameworks.
    #[serde(rename = "frameworks", default)]
    pub frameworks: Vec<crate::models::Framework>,
    /// Total number of frameworks that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
