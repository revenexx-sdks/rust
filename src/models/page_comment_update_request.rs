use serde::{Deserialize, Serialize};

/// The new body. Nothing else about a comment is editable.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageCommentUpdateRequest {
    /// The comment, as editor HTML. Replaces the old body completely.
    #[serde(rename = "body", default)]
    pub body: String,
}
