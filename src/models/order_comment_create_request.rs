use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderCommentCreateRequest {
    #[serde(rename = "author", default)]
    pub author: String,
    #[serde(rename = "body", default)]
    pub body: String,
    /// Default 'internal'.
    #[serde(rename = "visibility", default)]
    pub visibility: String,
}
