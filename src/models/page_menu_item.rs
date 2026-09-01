use serde::{Deserialize, Serialize};

/// One entry of a navigation menu. Stored verbatim, so a theme may carry extra
/// keys of its own alongside these.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageMenuItem {
    /// Sub-entries. This is how a two-level main navigation or a grouped footer is
    /// stored.
    #[serde(rename = "items", default)]
    pub items: Vec<serde_json::Value>,
    /// The words a visitor clicks.
    #[serde(rename = "label", default)]
    pub label: String,
    /// Where the entry goes: a page slug this app serves, a path the theme routes,
    /// or an absolute URL to somewhere else.
    #[serde(rename = "to", default)]
    pub to: String,
}
