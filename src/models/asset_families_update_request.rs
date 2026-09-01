use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssetFamiliesUpdateRequest {
    /// The asset family's stable identifier — a class of media with one shared
    /// shape. Unique per tenant.
    #[serde(rename = "code", default)]
    pub code: String,
    /// What the asset family is called, per language tag.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// How a file of this family is named, so an import can bind a file to a
    /// product without a mapping table. `source` is the product value the file
    /// name is built from, `pattern` how it is assembled, `allowed_extensions`
    /// what may be uploaded.
    #[serde(rename = "naming_convention", default)]
    pub naming_convention: serde_json::Value,
}
