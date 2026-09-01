use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectionList {
    /// Public collection names the tenant owns. These are the values accepted for
    /// the `collection` path parameter.
    #[serde(rename = "collections", default)]
    pub collections: Vec<String>,
}
