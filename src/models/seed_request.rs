use serde::{Deserialize, Serialize};

/// A theme's starting content. Both lists are optional; sending neither is a
/// no-op.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SeedRequest {
    /// The menus to create. One with no key or no label is reported under
    /// `skipped`.
    #[serde(rename = "menus", default)]
    pub menus: Vec<serde_json::Value>,
    /// The pages to create. One that has no `slug` or no `title` is reported under
    /// `skipped` rather than refused, so one bad entry never loses the rest.
    #[serde(rename = "pages", default)]
    pub pages: Vec<serde_json::Value>,
}
