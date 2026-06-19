use serde::{Deserialize, Serialize};

/// blökkli MutationResponseLike: success flag plus the full re-materialized
/// editor state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MutationResponse {
    /// Full editor state (see pages.editor.state).
    #[serde(rename = "state", default)]
    pub state: serde_json::Value,
    #[serde(rename = "success", default)]
    pub success: bool,
    #[serde(rename = "violations", default)]
    pub violations: Vec<serde_json::Value>,
}
