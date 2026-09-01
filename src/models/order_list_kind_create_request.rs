use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderListKindCreateRequest {
    /// What `lists.kind` will store. Lowercased on the way in and immutable
    /// afterwards — a merchant who wants a different code creates a new kind and
    /// moves the lists over.
    #[serde(rename = "code", default)]
    pub code: String,
    /// What this kind is for, in one sentence — the line a select shows under
    /// the title.
    #[serde(rename = "description", default)]
    pub description: String,
    /// Localized descriptions, keyed by language tag.
    #[serde(rename = "descriptions", default)]
    pub descriptions: serde_json::Value,
    /// Promote this kind; the previous default is demoted.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Localized titles, keyed by language tag.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Where the kind sits in a select, ascending. Omitted means 0, which puts it
    /// first among the unpositioned.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// What a person reads. `labels` adds the localized forms on top; this one is
    /// the fallback.
    #[serde(rename = "title", default)]
    pub title: String,
    /// Semantic badge colour. The client owns what each tone looks like; omitted
    /// means `neutral`.
    #[serde(rename = "tone", default)]
    pub tone: String,
}
