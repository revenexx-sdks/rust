use serde::{Deserialize, Serialize};

/// Executions List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutionList {
    /// List of executions.
    #[serde(rename = "executions", default)]
    pub executions: Vec<crate::models::Execution>,
    /// Total number of executions that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
