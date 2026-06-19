use serde::{Deserialize, Serialize};

/// Attributes List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeList {
    /// List of attributes.
    #[serde(rename = "attributes", default)]
    pub attributes: Vec<serde_json::Value>,
    /// Total number of attributes in the given collection.
    #[serde(rename = "total", default)]
    pub total: i64,
}
