use serde::{Deserialize, Serialize};

/// AttributeRelationship
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeRelationship {
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
    /// How deleting the parent document will propagate to child documents.
    #[serde(rename = "onDelete", default)]
    pub on_delete: String,
    /// The ID of the related collection.
    #[serde(rename = "relatedCollection", default)]
    pub related_collection: String,
    /// The type of the relationship.
    #[serde(rename = "relationType", default)]
    pub relation_type: String,
    /// Is attribute required?
    #[serde(rename = "required", default)]
    pub required: bool,
    /// Whether this is the parent or child side of the relationship
    #[serde(rename = "side", default)]
    pub side: String,
    /// Attribute status. Possible values: `available`, `processing`, `deleting`,
    /// `stuck`, or `failed`
    #[serde(rename = "status", default)]
    pub status: String,
    /// Is the relationship two-way?
    #[serde(rename = "twoWay", default)]
    pub two_way: bool,
    /// The key of the two-way relationship.
    #[serde(rename = "twoWayKey", default)]
    pub two_way_key: String,
    /// Attribute type.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
