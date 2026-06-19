use serde::{Deserialize, Serialize};

/// Deployments List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeploymentList {
    /// List of deployments.
    #[serde(rename = "deployments", default)]
    pub deployments: Vec<crate::models::Deployment>,
    /// Total number of deployments that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
