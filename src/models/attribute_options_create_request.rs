use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeOptionsCreateRequest {
    /// The select / multi-select attribute these are the permitted values of.
    /// Deleting the attribute deletes its options with it.
    #[serde(rename = "attribute_id", default)]
    pub attribute_id: String,
    /// The value actually STORED in a record's `attribute_values` when this option
    /// is picked — never the label. Unique within the attribute.
    #[serde(rename = "code", default)]
    pub code: String,
    /// What the option is called, per language tag. Two tenants may label the same
    /// code differently; only the code is ever written into a record.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Order in the dropdown, ascending. Options that tie keep the order the
    /// database returns them in, so give every option a position if the order
    /// matters.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// A colour or texture chip for the picker. Null for an option that is not
    /// visual.
    #[serde(rename = "swatch", default)]
    pub swatch: serde_json::Value,
}
