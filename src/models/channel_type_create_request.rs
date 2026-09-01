use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelTypeCreateRequest {
    /// What `channels.type` will store. Lowercased and trimmed before it is
    /// written, and fixed from then on — a rename would orphan every channel
    /// carrying it.
    #[serde(rename = "code", default)]
    pub code: String,
    /// One sentence on what kind of place this type of channel is, for the
    /// merchant choosing between them. Plain text, in the tenant's primary
    /// language; `descriptions` carries the per-locale ones.
    #[serde(rename = "description", default)]
    pub description: String,
    /// A locale map keyed by language tag: {"en": …, "de": …}. Read the
    /// requested tag and fall back to the plain column beside it.
    #[serde(rename = "descriptions", default)]
    pub descriptions: serde_json::Value,
    /// Promote this type; the previous default is demoted. The default is the type
    /// a channel created without one gets.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// A locale map keyed by language tag: {"en": …, "de": …}. Read the
    /// requested tag and fall back to the plain column beside it.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Sort position (default 0). GET /channels/types answers in this order; ties
    /// fall back to the code.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The fallback name. `labels` carries the per-locale ones.
    #[serde(rename = "title", default)]
    pub title: String,
    /// Badge colour (default 'neutral'). A value outside the palette is ignored
    /// rather than refused.
    #[serde(rename = "tone", default)]
    pub tone: String,
}
