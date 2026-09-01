use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderCommentCreateRequest {
    /// Who wrote it, as the caller reported it. Free text; not resolved against a
    /// user directory.
    #[serde(rename = "author", default)]
    pub author: String,
    /// The comment itself. Plain text; this app neither renders nor sanitizes it.
    #[serde(rename = "body", default)]
    pub body: String,
    /// Who may see it: 'internal' is a note between operators, 'customer' is meant
    /// to be shown in the customer's order view. Nothing here enforces that —
    /// this app labels the comment and the client showing it decides. Defaults to
    /// the tenant's default_comment_visibility. Defaults to the tenant's
    /// default_comment_visibility setting, which is 'internal' out of the box.
    #[serde(rename = "visibility", default)]
    pub visibility: String,
}
