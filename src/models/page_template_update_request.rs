use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageTemplateUpdateRequest {
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "field_name", default)]
    pub field_name: String,
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    #[serde(rename = "label", default)]
    pub label: String,
    #[serde(rename = "page_bundle", default)]
    pub page_bundle: String,
    /// Serialized block trees ({ bundle, props, props_i18n, options, children }).
    #[serde(rename = "tree", default)]
    pub tree: Vec<serde_json::Value>,
}
