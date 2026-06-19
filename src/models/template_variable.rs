use serde::{Deserialize, Serialize};

/// Template Variable
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemplateVariable {
    /// Variable Description.
    #[serde(rename = "description", default)]
    pub description: String,
    /// Variable Name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Variable Placeholder.
    #[serde(rename = "placeholder", default)]
    pub placeholder: String,
    /// Is the variable required?
    #[serde(rename = "required", default)]
    pub required: bool,
    /// Variable secret flag. Secret variables can only be updated or deleted, but
    /// never read.
    #[serde(rename = "secret", default)]
    pub secret: bool,
    /// Variable Type.
    #[serde(rename = "type", default)]
    pub xtype: String,
    /// Variable Value.
    #[serde(rename = "value", default)]
    pub value: String,
}
