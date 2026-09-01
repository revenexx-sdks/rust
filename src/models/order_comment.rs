use serde::{Deserialize, Serialize};

/// A note on an order, either internal between operators or meant for the
/// customer to see.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderComment {
    /// Who wrote it, as the caller reported it. Free text; not resolved against a
    /// user directory.
    #[serde(rename = "author", default)]
    pub author: String,
    /// The comment itself. Plain text; this app neither renders nor sanitizes it.
    #[serde(rename = "body", default)]
    pub body: String,
    /// When the comment was written. Comments come back oldest first.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Primary key of the comment.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The order the comment hangs on.
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    /// Who may see it: 'internal' is a note between operators, 'customer' is meant
    /// to be shown in the customer's order view. Nothing here enforces that —
    /// this app labels the comment and the client showing it decides. Defaults to
    /// the tenant's default_comment_visibility.
    #[serde(rename = "visibility", default)]
    pub visibility: String,
}
