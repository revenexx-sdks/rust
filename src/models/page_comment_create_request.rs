use serde::{Deserialize, Serialize};

/// A new comment. Send `blockUuids` for a thread anchored to blocks,
/// `parentUuid` for a reply.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageCommentCreateRequest {
    /// The blocks this thread is about, so the editor can draw a marker next to
    /// them. Leave empty for a comment about the page as a whole.
    #[serde(rename = "blockUuids", default)]
    pub block_uuids: Vec<String>,
    /// The comment, as editor HTML. `<span data-type="mention" data-id="USER_ID">`
    /// is what this app reads to decide whom to notify; `<li data-type="taskItem"
    /// data-checked="false">` makes a checkbox the toggle-task route can flip.
    #[serde(rename = "body", default)]
    pub body: String,
    /// The root comment this replies to. Omit for a new thread — only roots can
    /// be resolved.
    #[serde(rename = "parentUuid", default)]
    pub parent_uuid: String,
}
