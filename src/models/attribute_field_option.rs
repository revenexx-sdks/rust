use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeFieldOption {
    /// What to show in the picker, already resolved for the requested locale.
    #[serde(rename = "label", default)]
    pub label: String,
    /// Colour/texture chip, when the option carries one — `{"hex": "#c0c0c0"}`.
    #[serde(rename = "swatch", default)]
    pub swatch: serde_json::Value,
    /// The stored value — an `attribute_options.code`, or a
    /// `reference_entity_records.code` when the options ARE a reference entity.
    /// This, never the label, is what goes into `attribute_values`.
    #[serde(rename = "value", default)]
    pub value: String,
}
