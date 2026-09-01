use serde::{Deserialize, Serialize};

/// The row is gone. Deleting is not idempotent here: a second call answers
/// 404, because the row no longer resolves.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderDeleted {
    /// Always true — a failed delete is a status code, not a false here.
    #[serde(rename = "deleted", default)]
    pub deleted: bool,
    /// The id of the row that was deleted, echoed back.
    #[serde(rename = "id", default)]
    pub id: String,
}
