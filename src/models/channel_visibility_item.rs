use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelVisibilityItem {
    /// The row's channel scope slugs. Empty or absent means unassigned — the
    /// case the policy decides.
    #[serde(rename = "channels", default)]
    pub channels: Vec<String>,
    /// The row id, echoed back on the decision. Opaque to this app — it is never
    /// looked up, so any non-empty string is accepted and nothing has to exist. In
    /// practice it is the entity id POST /api/v1/scopes/lookup answered with,
    /// which is what the example shows.
    #[serde(rename = "id", default)]
    pub id: String,
}
