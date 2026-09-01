use serde::{Deserialize, Serialize};

/// Where to put the undo pointer.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageHistoryRequest {
    /// The position in the mutation log to materialize at. `-1` undoes everything;
    /// the last position redoes everything. Values outside the log are clamped
    /// rather than refused.
    #[serde(rename = "index", default)]
    pub index: i64,
    /// Which language the returned state should be resolved for.
    #[serde(rename = "langcode", default)]
    pub langcode: String,
}
