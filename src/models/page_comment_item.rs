use serde::{Deserialize, Serialize};

/// One comment, in the shape the editor renders — this is not the stored
/// row: the id is `uuid`, the timestamps are `created`/`updated` and the
/// author is nested under `user`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageCommentItem {
    /// The blocks this thread hangs on, so the editor can draw a marker next to
    /// them. Empty for a comment about the page as a whole.
    #[serde(rename = "blockUuids", default)]
    pub block_uuids: Vec<String>,
    /// The comment itself, as editor HTML. @mentions are `<span
    /// data-type="mention" data-id="…">` — that is what this app reads to
    /// decide whom to notify — and task checkboxes are `<li data-type="taskItem"
    /// data-checked="…">`.
    #[serde(rename = "body", default)]
    pub body: String,
    /// When the comment was written.
    #[serde(rename = "created", default)]
    pub created: String,
    /// The root comment this is a reply to. Absent on a root — and only roots
    /// can be resolved.
    #[serde(rename = "parentUuid", default)]
    pub parent_uuid: String,
    /// Whether the thread was marked done. Replies inherit nothing: resolving is a
    /// property of the root.
    #[serde(rename = "resolved", default)]
    pub resolved: bool,
    /// When it was last edited. Absent when it never was.
    #[serde(rename = "updated", default)]
    pub updated: String,
    /// Who wrote it, or `null` when it was written without an identity.
    #[serde(rename = "user", default)]
    pub user: serde_json::Value,
    /// The comment id. Every comment route addresses one by it.
    #[serde(rename = "uuid", default)]
    pub uuid: String,
}
