use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormDeleteResult {
    /// True when the policy is 'archive' and submissions exist — the form was
    /// archived, not deleted.
    #[serde(rename = "archived", default)]
    pub archived: bool,
    /// The form row was removed — and with it, via the cascade, every submission
    /// it had. `submissions` below says how many went, and they are not
    /// recoverable.
    #[serde(rename = "deleted", default)]
    pub deleted: bool,
    /// The form in the path.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The form's status after the call. Only present on the archive branch.
    #[serde(rename = "status", default)]
    pub status: String,
    /// How many submissions the form had when the call was weighed — and
    /// therefore, when `deleted` is true, how many were deleted with it. The whole
    /// inbox, across every market: the cascade is a database operation and takes
    /// them all, so an active `X-Revenexx-Market` does not narrow this number.
    #[serde(rename = "submissions", default)]
    pub submissions: i64,
}
