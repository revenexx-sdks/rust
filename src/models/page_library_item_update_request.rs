use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageLibraryItemUpdateRequest {
    #[serde(rename = "bundle", default)]
    pub bundle: String,
    #[serde(rename = "label", default)]
    pub label: String,
    /// Serialized block tree ({ bundle, props, props_i18n, options, children }).
    #[serde(rename = "tree", default)]
    pub tree: serde_json::Value,
}
