use serde::{Deserialize, Serialize};

/// Every comment of the page, roots and replies flat in one list, oldest first
/// — the editor builds the threads from `parentUuid`. Every write route
/// answers this same full list rather than the row it changed.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageCommentList {
    /// The page's comments, oldest first.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::PageCommentItem>,
}
