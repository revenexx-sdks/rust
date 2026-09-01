use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeSchemaGroup {
    /// The group code, which is what every field in the section carries as its
    /// `group`.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The section heading, resolved for the requested locale.
    #[serde(rename = "label", default)]
    pub label: String,
    /// Where the section sits, ascending. The array is already in this order.
    #[serde(rename = "position", default)]
    pub position: i64,
}
