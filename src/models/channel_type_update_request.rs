use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelTypeUpdateRequest {
    /// Replace the one-sentence description. Sent as null it is cleared; omitted
    /// it is kept. `descriptions` carries the per-locale ones.
    #[serde(rename = "description", default)]
    pub description: String,
    /// A locale map keyed by language tag: {"en": …, "de": …}. Read the
    /// requested tag and fall back to the plain column beside it.
    #[serde(rename = "descriptions", default)]
    pub descriptions: serde_json::Value,
    /// Promote this type; the previous default is demoted. Only `true` does
    /// anything — sending false does not demote this type, because some type
    /// must hold the flag.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// A locale map keyed by language tag: {"en": …, "de": …}. Read the
    /// requested tag and fall back to the plain column beside it.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Move the type in the order GET /channels/types answers in.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// Rename the type. A blank or non-string title is ignored, not refused —
    /// the stored one is kept.
    #[serde(rename = "title", default)]
    pub title: String,
    /// Change the badge colour. A value outside the palette is ignored rather than
    /// refused, and the stored tone is kept.
    #[serde(rename = "tone", default)]
    pub tone: String,
}
