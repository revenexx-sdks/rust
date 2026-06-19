use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductTaxRef {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "sku", default)]
    pub sku: String,
    #[serde(rename = "tax_class", default)]
    pub tax_class: String,
}
