use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelVisibilityDecision {
    /// The id as it was sent, verbatim.
    #[serde(rename = "id", default)]
    pub id: String,
    /// Why the row was shown or hidden — the answer is auditable, not a bare
    /// boolean.
    #[serde(rename = "reason", default)]
    pub reason: String,
    /// Whether this row may be shown in the resolved channel. The same answer as
    /// membership in `visible`; `reason` says why.
    #[serde(rename = "visible", default)]
    pub visible: bool,
}
