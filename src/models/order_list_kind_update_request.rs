use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderListKindUpdateRequest {
    /// What this kind is for, in one sentence. Explicit null clears it.
    #[serde(rename = "description", default)]
    pub description: String,
    /// Localized descriptions, keyed by language tag. Replaces the whole map
    /// rather than merging into it.
    #[serde(rename = "descriptions", default)]
    pub descriptions: serde_json::Value,
    /// True promotes this kind and demotes the previous default — the same move
    /// POST /orderlists/kinds/{id}/make-default makes on its own.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Localized titles, keyed by language tag. Replaces the whole map rather than
    /// merging into it.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Where the kind sits in a select, ascending.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// What a person reads. A blank title is ignored rather than stored — a kind
    /// with no words is unreadable in every UI.
    #[serde(rename = "title", default)]
    pub title: String,
    /// Semantic badge colour. The client owns what each tone looks like.
    #[serde(rename = "tone", default)]
    pub tone: String,
}
