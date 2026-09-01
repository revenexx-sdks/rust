use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value. At least one
/// field is required.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelUpdateRequest {
    /// Stable channel code, unique per tenant (e.g. shop, punchout-acme). It is
    /// the scope slug Baseline matches channel assignments on, so it is held to
    /// Baseline's own shape: lowercase a-z/0-9 first, then a-z/0-9/_/-, up to 63
    /// characters. Anything else is refused — a code that cannot be a scope slug
    /// leaves the channel unable to filter.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Mark as the default channel (default false). At most one channel carries it
    /// — setting it demotes the previous holder.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Localized display names. A locale map keyed by language tag: {"en": …,
    /// "de": …}. Read the requested tag and fall back to the plain column beside
    /// it.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Display name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sort position (default 0).
    #[serde(rename = "position", default)]
    pub position: i64,
    /// Lifecycle status (default 'active'). Whether the channel is in service.
    /// What 'inactive' DOES is the tenant's inactive_channel_behavior setting: on
    /// 'serve' it is a label and the channel still resolves, on 'block'
    /// /channels/context answers resolved:false with reason 'channel_inactive'.
    /// Served as the 'channels.statuses' vocabulary.
    #[serde(rename = "status", default)]
    pub status: String,
    /// Which channel type this is. One of the codes the tenant keeps under GET
    /// /channels/types — served with labels as the 'channels.types' vocabulary.
    /// Deliberately NOT an enum: the set is the tenant's own rows, not a CHECK
    /// constraint this repo could quote. A fresh install starts with storefront,
    /// punchout, marketplace, api, pos, which is why 'storefront' is the example
    /// here, but a merchant may rename or retire any of them and add their own (a
    /// feed or a print channel), so read the list rather than assuming it. Omitted
    /// on create it falls back to the type the tenant flagged as their default,
    /// never to a hardcoded value; a code the tenant does not keep is a 400 that
    /// names the ones they do.
    #[serde(rename = "type", default)]
    pub xtype: String,
    /// Default 'inherit'. What it means, IN THIS CHANNEL, that a row carries no
    /// channel assignment at all — the per-channel override of the tenant-wide
    /// unassigned_channel_visibility setting. 'inherit' (the default) takes the
    /// tenant's answer and changes nothing. 'all' shows unassigned rows:
    /// everything is on sale unless somebody carved it out, which is what an open
    /// storefront wants and what Baseline's is_visible() does today.
    /// 'assigned_only' hides them until they are explicitly assigned — the
    /// negotiated assortment a punchout contract describes, and the one answer the
    /// generated _scoped view has no way to express, which is why POST
    /// /channels/visibility exists to apply it. Rows that DO carry assignments are
    /// unaffected either way. Served with its labels as the
    /// 'channels.unassigned-visibility' vocabulary.
    #[serde(rename = "unassigned_visibility", default)]
    pub unassigned_visibility: String,
}
