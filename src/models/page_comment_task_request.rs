use serde::{Deserialize, Serialize};

/// Which checkbox to flip.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageCommentTaskRequest {
    /// The task item to toggle, counted in document order from 0. A comment with
    /// fewer tasks than that answers 400, and so does anything that is not a whole
    /// number at or above 0.
    #[serde(rename = "taskIndex", default)]
    pub task_index: i64,
}
