use serde::{Deserialize, Serialize};

/// Specifications List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpecificationList {
    /// List of specifications.
    #[serde(rename = "specifications", default)]
    pub specifications: Vec<crate::models::Specification>,
    /// Total number of specifications that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
