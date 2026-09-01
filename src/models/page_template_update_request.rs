use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value. A template is a
/// COPY source, so changing it never reaches the pages already made from it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageTemplateUpdateRequest {
    /// A sentence about when to reach for it, shown next to the label.
    #[serde(rename = "description", default)]
    pub description: String,
    /// The field this template is offered in. Null offers it in every field.
    #[serde(rename = "field_name", default)]
    pub field_name: String,
    /// Whether a new page of this bundle starts from this template.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// What the template is called in the picker.
    #[serde(rename = "label", default)]
    pub label: String,
    /// The page type this template is offered on. Null offers it on every page
    /// type.
    #[serde(rename = "page_bundle", default)]
    pub page_bundle: String,
    /// The blocks the template inserts, in order. Replaces the stored tree
    /// completely.
    #[serde(rename = "tree", default)]
    pub tree: Vec<crate::models::PageBlockTree>,
}
