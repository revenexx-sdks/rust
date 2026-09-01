use serde::{Deserialize, Serialize};

/// The family the fields belong to, or null when none was named — then the
/// answer is every attribute of the `entity_type`, which is what a reference
/// entity or an asset family has instead of a family.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeSchemaFamily {
    /// The family's code — the value `?family_code=` takes.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The family's id.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The family name, resolved for the requested locale.
    #[serde(rename = "label", default)]
    pub label: String,
    /// Which of these fields is the product's display name.
    #[serde(rename = "label_attribute", default)]
    pub label_attribute: String,
}
