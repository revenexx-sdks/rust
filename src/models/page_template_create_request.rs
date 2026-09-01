use serde::{Deserialize, Serialize};

/// The blocks to freeze, and where the template should be offered.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageTemplateCreateRequest {
    /// A sentence about when to reach for it.
    #[serde(rename = "description", default)]
    pub description: String,
    /// The field this template should be offered in. Null offers it in every
    /// field.
    #[serde(rename = "fieldName", default)]
    pub field_name: String,
    /// Whether a new page of that type should start from this template.
    #[serde(rename = "isDefault", default)]
    pub is_default: bool,
    /// What the template is called in the picker.
    #[serde(rename = "label", default)]
    pub label: String,
    /// The page type this template should be offered on. Omit to take the current
    /// page's own type.
    #[serde(rename = "pageBundle", default)]
    pub page_bundle: String,
    /// The blocks to serialize into the template, each with its whole subtree.
    /// They are read from the CURRENT edit state, so unpublished changes are
    /// included.
    #[serde(rename = "uuids", default)]
    pub uuids: Vec<String>,
}
