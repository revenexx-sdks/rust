use serde::{Deserialize, Serialize};

/// Which entry of the history to switch, and to what.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageMutationStatusRequest {
    /// Whether the entry takes part in the replay.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// The position in the mutation log to switch. Unknown positions answer 404.
    #[serde(rename = "index", default)]
    pub index: i64,
    /// Which language the returned state should be resolved for.
    #[serde(rename = "langcode", default)]
    pub langcode: String,
}
