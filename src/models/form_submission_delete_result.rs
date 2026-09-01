use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormSubmissionDeleteResult {
    /// Always true — the row is gone. A submission that was not there answers
    /// 404 instead, so this is never false.
    #[serde(rename = "deleted", default)]
    pub deleted: bool,
    /// The submission that was removed, echoed from the path.
    #[serde(rename = "id", default)]
    pub id: String,
}
