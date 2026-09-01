use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductTaxRef {
    /// The product's id.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The product's resolved display name, or its SKU when the catalog holds no
    /// name for it.
    #[serde(rename = "label", default)]
    pub label: String,
    /// The SKU, so a caller that asked by id can key its own answer by SKU and the
    /// other way round.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// The tax class key the prices app resolves a rate from. Null means the
    /// product names none and the caller has to fall back to its own default.
    #[serde(rename = "tax_class", default)]
    pub tax_class: String,
}
