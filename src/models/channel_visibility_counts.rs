use serde::{Deserialize, Serialize};

/// The three tallies, so a caller can log or alert on a batch without walking
/// it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelVisibilityCounts {
    /// How many must not be. A batch where this equals `total` and the reason is
    /// no_channel_context means the channel did not resolve, not that the
    /// assortment is empty.
    #[serde(rename = "hidden", default)]
    pub hidden: i64,
    /// How many rows were decided — the length of the `items` sent.
    #[serde(rename = "total", default)]
    pub total: i64,
    /// How many may be shown.
    #[serde(rename = "visible", default)]
    pub visible: i64,
}
