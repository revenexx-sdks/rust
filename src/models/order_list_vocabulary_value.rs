use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderListVocabularyValue {
    /// A plain string, or a locale map keyed by language tag ({"en": …, "de":
    /// …}). Read the requested tag, fall back to `en`.
    #[serde(rename = "description", default)]
    pub description: serde_json::Value,
    /// Localized descriptions of a tenant-owned value, keyed by locale.
    #[serde(rename = "descriptions", default)]
    pub descriptions: serde_json::Value,
    /// The value ends the lifecycle. Always false for `kinds` — a list kind is
    /// not a state.
    #[serde(rename = "final", default)]
    pub xfinal: bool,
    /// The value a create falls back to, so a client can mark it without reading
    /// the settings as well.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Seeded on install rather than created by the tenant. Still renameable and
    /// retirable.
    #[serde(rename = "is_system", default)]
    pub is_system: bool,
    /// The value as the database stores and enforces it — for `kinds`, the
    /// `code` a list carries.
    #[serde(rename = "key", default)]
    pub key: String,
    /// Localized titles of a tenant-owned value, keyed by locale.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// A plain string, or a locale map keyed by language tag ({"en": …, "de":
    /// …}). Read the requested tag, fall back to `en`.
    #[serde(rename = "title", default)]
    pub title: serde_json::Value,
    /// Semantic badge colour. The client owns what each tone looks like.
    #[serde(rename = "tone", default)]
    pub tone: String,
}
