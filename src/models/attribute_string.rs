use serde::{Deserialize, Serialize};

/// AttributeString
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeString {
    /// Attribute creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Attribute update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Is attribute an array?
    #[serde(rename = "array", default)]
    pub array: bool,
    /// Defines whether this attribute is encrypted or not.
    #[serde(rename = "encrypt", default)]
    pub encrypt: bool,
    /// Error message. Displays error generated on failure of creating or deleting
    /// an attribute.
    #[serde(rename = "error", default)]
    pub error: String,
    /// Attribute Key.
    #[serde(rename = "key", default)]
    pub key: String,
    /// Is attribute required?
    #[serde(rename = "required", default)]
    pub required: bool,
    /// Attribute size.
    #[serde(rename = "size", default)]
    pub size: i64,
    /// Attribute status. Possible values: `available`, `processing`, `deleting`,
    /// `stuck`, or `failed`
    #[serde(rename = "status", default)]
    pub status: String,
    /// Attribute type.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
