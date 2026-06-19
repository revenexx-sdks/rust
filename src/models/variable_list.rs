use serde::{Deserialize, Serialize};

/// Variables List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VariableList {
    /// Total number of variables that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
    /// List of variables.
    #[serde(rename = "variables", default)]
    pub variables: Vec<crate::models::Variable>,
}
