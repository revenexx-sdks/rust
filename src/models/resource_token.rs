use serde::{Deserialize, Serialize};

/// ResourceToken
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceToken {
    /// Token creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Token ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Most recent access date in ISO 8601 format. This attribute is only updated
    /// again after 24 hours.
    #[serde(rename = "accessedAt", default)]
    pub accessed_at: String,
    /// Token expiration date in ISO 8601 format.
    #[serde(rename = "expire", default)]
    pub expire: String,
    /// Resource ID.
    #[serde(rename = "resourceId", default)]
    pub resource_id: String,
    /// Resource type.
    #[serde(rename = "resourceType", default)]
    pub resource_type: String,
    /// JWT encoded string.
    #[serde(rename = "secret", default)]
    pub secret: String,
}
