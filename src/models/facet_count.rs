use serde::{Deserialize, Serialize};

/// Facet values and their counts for one faceted field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FacetCount {
    #[serde(rename = "counts", default)]
    pub counts: Vec<serde_json::Value>,
    #[serde(rename = "field_name", default)]
    pub field_name: String,
}
