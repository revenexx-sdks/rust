use serde::{Deserialize, Serialize};

/// One value of the address types set. What an address is used for. Billing
/// and shipping are what a checkout needs; a works entrance or a central
/// accounts office is the tenant's own.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddressTypeRow {
    /// What `addresses.type` stores, and the only part of this row other data
    /// depends on. Immutable once created: renaming it would orphan every record
    /// carrying it.
    #[serde(rename = "code", default)]
    pub code: String,
    /// When the value was added to this set.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// One line of help for an operator choosing this value. Null when there is
    /// nothing to add. A row seeded before 0.22.0 may hold a serialized locale map
    /// here instead (PE-443).
    #[serde(rename = "description", default)]
    pub description: String,
    /// Localized descriptions, keyed by language tag ({ "en": …, "de": … }).
    /// Null when nobody translated this value — a client then falls back to
    /// `description`.
    #[serde(rename = "descriptions", default)]
    pub descriptions: serde_json::Value,
    /// Primary key of this value. What the update and delete routes address it by
    /// — the CODE is what records store.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The value a create falls back to when the caller names none. Exactly one
    /// row of the set carries it; promoting another one demotes this.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// True for a value this app seeded on install. Still renameable and still
    /// removable — it only records where the value came from.
    #[serde(rename = "is_system", default)]
    pub is_system: bool,
    /// Localized titles, keyed by language tag ({ "en": …, "de": … }). Null
    /// when nobody translated this value — a client then falls back to `title`.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Where this value sits in the set, ascending. It is the order a select
    /// should offer.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The tenant this row belongs to — the store slug, not an id. Set by the
    /// platform from the authenticated context, never by a caller; a write that
    /// carries it is ignored, and no request can read another tenant's rows by
    /// sending a different one.
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    /// The fallback name — what a client shows when no locale in `labels`
    /// matches. A row seeded before 0.22.0 may hold a serialized locale map here
    /// instead (PE-443) — those rows were seeded with no `labels` at all.
    #[serde(rename = "title", default)]
    pub title: String,
    /// Semantic badge colour. The palette stays fixed — it is a render concern,
    /// not a merchant decision.
    #[serde(rename = "tone", default)]
    pub tone: String,
    /// When it was last edited.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
