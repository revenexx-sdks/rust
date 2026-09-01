use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoryRuleSample {
    /// A matching product.
    #[serde(rename = "id", default)]
    pub id: String,
    /// Its SKU, so the sample is readable. Null only for a row whose SKU is unset,
    /// which the database does not allow.
    #[serde(rename = "sku", default)]
    pub sku: String,
}
