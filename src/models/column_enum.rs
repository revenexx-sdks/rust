use serde::{Deserialize, Serialize};

/// ColumnEnum
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ColumnEnum {
    /// Column creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Column update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Is column an array?
    #[serde(rename = "array", default)]
    pub array: bool,
    /// Array of elements in enumerated type.
    #[serde(rename = "elements", default)]
    pub elements: Vec<String>,
    /// Error message. Displays error generated on failure of creating or deleting
    /// an column.
    #[serde(rename = "error", default)]
    pub error: String,
    /// String format.
    #[serde(rename = "format", default)]
    pub format: String,
    /// Column Key.
    #[serde(rename = "key", default)]
    pub key: String,
    /// Is column required?
    #[serde(rename = "required", default)]
    pub required: bool,
    /// Column status. Possible values: `available`, `processing`, `deleting`,
    /// `stuck`, or `failed`
    #[serde(rename = "status", default)]
    pub status: String,
    /// Column type.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
