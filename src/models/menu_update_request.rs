use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value. `items` is
/// replaced wholesale when sent.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MenuUpdateRequest {
    /// The ordered navigation tree. Replaces the stored one completely.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::PageMenuItem>,
    /// What this menu is called for the people who edit it.
    #[serde(rename = "label", default)]
    pub label: String,
}
