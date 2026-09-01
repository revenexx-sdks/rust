use serde::{Deserialize, Serialize};

/// What was created and what was already there. Nothing is ever overwritten,
/// so a non-empty `skipped` is the normal answer to a second run.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SeedResult {
    /// The menu half of the run.
    #[serde(rename = "menus", default)]
    pub menus: serde_json::Value,
    /// The page half of the run.
    #[serde(rename = "pages", default)]
    pub pages: serde_json::Value,
}
