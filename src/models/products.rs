use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Products {
    #[serde(rename = "attribute_values", default)]
    pub attribute_values: serde_json::Value,
    #[serde(rename = "completeness", default)]
    pub completeness: serde_json::Value,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "deleted_at", default)]
    pub deleted_at: String,
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    #[serde(rename = "family_id", default)]
    pub family_id: String,
    #[serde(rename = "family_variant_id", default)]
    pub family_variant_id: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "kind", default)]
    pub kind: String,
    #[serde(rename = "parent_id", default)]
    pub parent_id: String,
    #[serde(rename = "quantified_associations", default)]
    pub quantified_associations: serde_json::Value,
    #[serde(rename = "sku", default)]
    pub sku: String,
    #[serde(rename = "tax_class", default)]
    pub tax_class: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
