use serde::{Deserialize, Serialize};

/// Variable
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Variable {
    /// Variable creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Variable ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Variable creation date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Variable key.
    #[serde(rename = "key", default)]
    pub key: String,
    /// ID of resource to which the variable belongs. If resourceType is "project",
    /// it is empty. If resourceType is "function", it is ID of the function.
    #[serde(rename = "resourceId", default)]
    pub resource_id: String,
    /// Service to which the variable belongs. Possible values are "project",
    /// "function"
    #[serde(rename = "resourceType", default)]
    pub resource_type: String,
    /// Variable secret flag. Secret variables can only be updated or deleted, but
    /// never read.
    #[serde(rename = "secret", default)]
    pub secret: bool,
    /// Variable value.
    #[serde(rename = "value", default)]
    pub value: String,
}
