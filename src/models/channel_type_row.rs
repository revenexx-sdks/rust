use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelTypeRow {
    /// What `channels.type` stores. Immutable once created — renaming it would
    /// orphan every channel that carries it, and there is no FK behind
    /// `channels.type` to cascade. A fresh install seeds storefront, punchout,
    /// marketplace, api, pos; a merchant may retire any of them and add their own.
    #[serde(rename = "code", default)]
    pub code: String,
    /// When the row was inserted, set by the database.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// A plain string, or a locale map keyed by language tag ({"en": …, "de":
    /// …}). Read the requested tag, fall back to `en`.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// A locale map keyed by language tag: {"en": …, "de": …}. Read the
    /// requested tag and fall back to the plain column beside it.
    #[serde(rename = "descriptions", default)]
    pub descriptions: serde_json::Value,
    /// Row id, and the only handle GET/PUT/DELETE /channels/types/{id} accept. Not
    /// the type `code`. No example is published because no id this app could
    /// invent names a row a tenant holds.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The type a channel created without one gets. Exactly one row carries it.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Seeded on install rather than added by the merchant. A flag about origin
    /// only — a system type is still renameable, reorderable and retirable.
    #[serde(rename = "is_system", default)]
    pub is_system: bool,
    /// A locale map keyed by language tag: {"en": …, "de": …}. Read the
    /// requested tag and fall back to the plain column beside it.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Sort position. GET /channels/types always answers in this order and takes
    /// no `order` parameter. It is not unique and defaults to 0, so ties are
    /// broken by `code` — the order is total, which is what makes paging the
    /// list safe to walk.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The tenant that owns this row. Added by the data plane, not by this app: it
    /// is not a column of schema.json, so it is read-only and `?tenant_id=` is not
    /// a filter — the key is silently dropped and never reaches the `filter`
    /// echo.
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    /// The fallback name. `labels` carries the per-locale ones. Rows seeded before
    /// 0.7.0 hold a serialized locale map here instead (PE-452).
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
    /// Semantic badge colour for this type, for a client that renders the list.
    /// The client owns what each tone looks like; the value only says what it
    /// MEANS.
    #[serde(rename = "tone", default)]
    pub tone: String,
    /// When the row was last written, set by the database.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
