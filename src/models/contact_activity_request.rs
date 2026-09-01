use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactActivityRequest {
    /// Who logged it (operator id or email). Free text; this app does not resolve
    /// it.
    #[serde(rename = "actor", default)]
    pub actor: String,
    /// What happened. 'system' is deliberately NOT accepted — those rows are the
    /// registration decision trail and are written by the approve/reject routes.
    /// Default 'note'.
    #[serde(rename = "kind", default)]
    pub kind: String,
    /// The long form. Stored inside the event payload as `note`, not as a column
    /// of its own.
    #[serde(rename = "note", default)]
    pub note: String,
    /// When it actually happened. Defaults to now — a call logged on Monday
    /// about Friday should say Friday.
    #[serde(rename = "occurred_at", default)]
    pub occurred_at: String,
    /// One line a person can scan in a timeline. Required — an entry nobody can
    /// read at a glance is not worth the row.
    #[serde(rename = "subject", default)]
    pub subject: String,
}
