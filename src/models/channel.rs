use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Channel {
    /// The scope slug Baseline matches channel assignments on
    /// (manifest.provides_scopes[].slug_source). Unique per tenant and, in
    /// practice, immutable — changing it orphans every assignment made against
    /// it.
    #[serde(rename = "code", default)]
    pub code: String,
    /// When the row was inserted, set by the database.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Row id, and the only handle GET/PUT/DELETE /channels/{id} accept. Not the
    /// scope slug — that is `code`. No example is published because no id this
    /// app could invent names a row a tenant holds.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The channel a request that names none falls back to. At most one channel
    /// carries it.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// A locale map keyed by language tag: {"en": …, "de": …}. Read the
    /// requested tag and fall back to the plain column beside it.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Display name. `labels` carries the per-locale ones.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sort position — ascending, and the tiebreak when two channels both claim
    /// is_default.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// Whether the channel is in service. What 'inactive' DOES is the tenant's
    /// inactive_channel_behavior setting: on 'serve' it is a label and the channel
    /// still resolves, on 'block' /channels/context answers resolved:false with
    /// reason 'channel_inactive'. Served as the 'channels.statuses' vocabulary.
    #[serde(rename = "status", default)]
    pub status: String,
    /// The tenant that owns this row. Added by the data plane, not by this app: it
    /// is not a column of schema.json, so it is read-only and `?tenant_id=` is not
    /// a filter — the key is silently dropped and never reaches the `filter`
    /// echo.
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    /// One of the codes the tenant keeps under GET /channels/types — served with
    /// labels as the 'channels.types' vocabulary. Deliberately NOT an enum: the
    /// set is the tenant's own rows, not a CHECK constraint this repo could quote.
    /// A fresh install starts with storefront, punchout, marketplace, api, pos,
    /// which is why 'storefront' is the example here, but a merchant may rename or
    /// retire any of them and add their own (a feed or a print channel), so read
    /// the list rather than assuming it.
    #[serde(rename = "type", default)]
    pub xtype: String,
    /// What it means, IN THIS CHANNEL, that a row carries no channel assignment at
    /// all — the per-channel override of the tenant-wide
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
    /// When the row was last written, set by the database.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
