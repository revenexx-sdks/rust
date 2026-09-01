use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelVisibility {
    /// The channel that resolved, or null. Null on every answer where `resolved`
    /// is false — including the everyday one on a tenant that has not created a
    /// channel yet.
    #[serde(rename = "channel", default)]
    pub channel: String,
    /// The three tallies, so a caller can log or alert on a batch without walking
    /// it.
    #[serde(rename = "counts", default)]
    pub counts: crate::models::ChannelVisibilityCounts,
    /// More than one channel claims is_default; the lowest position wins and this
    /// says so.
    #[serde(rename = "default_ambiguous", default)]
    pub default_ambiguous: bool,
    /// Just the ids that must NOT be shown. The complement of `visible`; together
    /// they are every id sent, so a caller can assert nothing was dropped.
    #[serde(rename = "hidden", default)]
    pub hidden: Vec<String>,
    /// One decision per row sent, in the order they were sent, so a caller can zip
    /// it back onto its own list without matching on id.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::ChannelVisibilityDecision>,
    /// The visibility policy in force for the resolved channel.
    #[serde(rename = "policy", default)]
    pub policy: crate::models::ChannelPolicy,
    /// Why not, when resolved is false. Null when it resolved.
    #[serde(rename = "reason", default)]
    pub reason: String,
    /// The channel code the request named, if any — lowercased and trimmed as it
    /// was matched.
    #[serde(rename = "requested", default)]
    pub requested: String,
    /// Whether a channel could be resolved for this request.
    #[serde(rename = "resolved", default)]
    pub resolved: bool,
    /// Where the channel came from, in the order they are tried: 'body' (the
    /// `channel` field, POST /channels/visibility only), 'query' (`?channel=`),
    /// 'header' (x-revenexx-channel), 'jwt' (the scope_context.channel claim),
    /// then 'default' (the channel flagged is_default). Null when nothing
    /// resolved. Note that 'header' is not reachable through api.revenexx.com: the
    /// gateway builds a fresh request to the app and copies a fixed set of headers
    /// into it, and x-revenexx-channel is not among them — see `policy.header`.
    #[serde(rename = "source", default)]
    pub source: String,
    /// Just the ids that may be shown, ready to filter a result set with — the
    /// same rows `items` marks visible:true, without the reasons.
    #[serde(rename = "visible", default)]
    pub visible: Vec<String>,
}
