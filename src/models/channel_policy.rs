use serde::{Deserialize, Serialize};

/// The visibility policy in force for the resolved channel.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelPolicy {
    /// Always 'channel' — the scope dimension this app provides.
    #[serde(rename = "dimension", default)]
    pub dimension: String,
    /// The header name Baseline uses for this dimension. Through api.revenexx.com
    /// it does NOT reach the app — the gateway builds a fresh request downstream
    /// and forwards only its own headers — so use `?channel=` (or `channel` in
    /// the body of POST /channels/visibility) instead. The header path applies to
    /// a direct in-cluster call to the app.
    #[serde(rename = "header", default)]
    pub header: String,
    /// The tenant setting, echoed: what `status = 'inactive'` DOES. 'serve' makes
    /// it a label and the channel still resolves; 'block' makes resolution fail
    /// with reason 'channel_inactive', and the policy then falls back to the
    /// tenant answer.
    #[serde(rename = "inactive_channel_behavior", default)]
    pub inactive_channel_behavior: String,
    /// The claim path in the forwarded identity token that names the active
    /// channel, tried after the query and the header and before the default
    /// channel.
    #[serde(rename = "jwt_path", default)]
    pub jwt_path: String,
    /// How Baseline matches the dimension — 'single': a request is in exactly
    /// one channel at a time, never a set.
    #[serde(rename = "match_mode", default)]
    pub match_mode: String,
    /// The tenant setting, echoed: whether a request naming no channel is refused
    /// rather than falling back to the default channel. On POST
    /// /channels/visibility that refusal is the single 400 this app makes of its
    /// own accord.
    #[serde(rename = "require_channel_context", default)]
    pub require_channel_context: bool,
    /// Whether the answer came from the tenant setting or this channel's own
    /// override. Only a channel that actually resolved gets a say — a blocked or
    /// unknown channel falls back to 'tenant'.
    #[serde(rename = "source", default)]
    pub source: String,
    /// The tenant-wide baseline, so a caller can see what this channel overrode.
    /// Equal to `unassigned_visibility` whenever `source` is 'tenant'.
    #[serde(rename = "tenant_default", default)]
    pub tenant_default: String,
    /// What a row with NO channel assignment means. 'all' is Baseline's
    /// open-by-default semantic, reproduced exactly; 'assigned_only' is the closed
    /// assortment the _scoped view cannot express.
    #[serde(rename = "unassigned_visibility", default)]
    pub unassigned_visibility: String,
}
