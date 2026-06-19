use serde::{Deserialize, Serialize};

/// AttributeFloat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeFloat {
    /// Attribute creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Attribute update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Is attribute an array?
    #[serde(rename = "array", default)]
    pub array: bool,
    /// Error message. Displays error generated on failure of creating or deleting
    /// an attribute.
    #[serde(rename = "error", default)]
    pub error: String,
    /// Attribute Key.
    #[serde(rename = "key", default)]
    pub key: String,
    /// Maximum value to enforce for new documents.
    #[serde(rename = "max", default)]
    pub max: f64,
    /// Minimum value to enforce for new documents.
    #[serde(rename = "min", default)]
    pub min: f64,
    /// Is attribute required?
    #[serde(rename = "required", default)]
    pub required: bool,
    /// Attribute status. Possible values: `available`, `processing`, `deleting`,
    /// `stuck`, or `failed`
    #[serde(rename = "status", default)]
    pub status: String,
    /// Attribute type.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
