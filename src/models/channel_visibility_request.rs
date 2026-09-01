use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelVisibilityRequest {
    /// The channel `code` (the scope slug) to evaluate against, trimmed and
    /// lowercased before it is matched. Optional, and through api.revenexx.com it
    /// is the ONLY way to name a channel explicitly: the x-revenexx-channel header
    /// is not forwarded to the app, so without this the resolution falls through
    /// to the scope_context.channel claim and then to the tenant's default
    /// channel. A code no channel carries is not an error — the answer is
    /// resolved:false with reason 'unknown_channel', so a caller can tell it from
    /// an outage.
    #[serde(rename = "channel", default)]
    pub channel: String,
    /// The rows to decide on, each with the channel assignments Baseline holds for
    /// it. POST /api/v1/scopes/lookup?dimension=channel answers in exactly this
    /// shape. At most 500 — Baseline's own lookup ceiling.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::ChannelVisibilityItem>,
}
