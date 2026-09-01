use serde::{Deserialize, Serialize};

/// The preferences to store for the calling user.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageUserSettingsRequest {
    /// The whole preferences bag — replaced, not merged, so send all of it. Its
    /// keys vary by the editor build and this app reads none of them. Null or
    /// omitted stores `{}`, which is how a user resets their editor.
    #[serde(rename = "settings", default)]
    pub settings: serde_json::Value,
}
