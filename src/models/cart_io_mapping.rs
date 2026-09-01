use serde::{Deserialize, Serialize};

/// Baseline-IO-compatible column mapping. An empty object (or null) is
/// identity: the full canonical shape, every field under its own name.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartIoMapping {
    /// Renames, in order. On export the row is narrowed to these columns; on
    /// import a column that is not listed is ignored. Omit or leave empty for
    /// identity.
    #[serde(rename = "columns", default)]
    pub columns: Vec<crate::models::CartIoMappingColumn>,
    /// Fields that identify a line in the payload — what the bundled quick-order
    /// template sets to ['sku'].
    #[serde(rename = "keys", default)]
    pub keys: Vec<String>,
}
