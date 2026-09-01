use serde::{Deserialize, Serialize};

/// A Typesense collection definition, passed through from Typesense. `name` is
/// rewritten back to the tenant's public collection name.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Collection {
    #[serde(rename = "default_sorting_field", default)]
    pub default_sorting_field: String,
    #[serde(rename = "enable_nested_fields", default)]
    pub enable_nested_fields: bool,
    #[serde(rename = "fields", default)]
    pub fields: Vec<crate::models::CollectionField>,
    /// The public collection name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Documents currently indexed.
    #[serde(rename = "num_documents", default)]
    pub num_documents: i64,
}
