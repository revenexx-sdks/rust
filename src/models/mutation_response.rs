use serde::{Deserialize, Serialize};

/// blökkli MutationResponseLike: whether the call was applied, plus the FULL
/// re-materialized editor state — so a client never has to re-fetch after a
/// change.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MutationResponse {
    /// Everything the blökkli editor runs on, for one page in one language,
    /// materialized at the current point of the undo history. The theme adapter
    /// maps it 1:1 onto blökkli's MappedState.
    #[serde(rename = "state", default)]
    pub state: crate::models::EditorState,
    /// Whether the change was applied.
    #[serde(rename = "success", default)]
    pub success: bool,
    /// Why the call was refused, when `success` is false.
    #[serde(rename = "violations", default)]
    pub violations: Vec<serde_json::Value>,
}
