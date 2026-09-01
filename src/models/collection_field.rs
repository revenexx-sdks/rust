use serde::{Deserialize, Serialize};

/// One field in a collection schema.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectionField {
    /// Whether the field can be faceted on.
    #[serde(rename = "facet", default)]
    pub facet: bool,
    #[serde(rename = "index", default)]
    pub index: bool,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "optional", default)]
    pub optional: bool,
    #[serde(rename = "sort", default)]
    pub sort: bool,
    /// Typesense field type, e.g. `string`, `int64`, `string[]`, `object`.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
