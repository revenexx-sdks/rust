use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductLabel {
    /// The attribute code the name was read from.
    #[serde(rename = "attribute", default)]
    pub attribute: String,
    /// How that attribute was chosen: 'family' is the product's own
    /// `families.label_attribute`, 'setting' the tenant's
    /// `default_label_attribute`, 'convention' the built-in fallback to `name`
    /// when neither says anything.
    #[serde(rename = "attribute_from", default)]
    pub attribute_from: String,
    /// The product's id.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The name to show. Never empty — read `source` before treating it as a
    /// name, because `sku` there means this is the SKU standing in for one.
    #[serde(rename = "label", default)]
    pub label: String,
    /// Which locale the value came out of, when it came from a locale bucket. Null
    /// for a value in `common` and for the SKU fallback.
    #[serde(rename = "locale", default)]
    pub locale: String,
    /// The SKU, which is also the fallback shown as `label` when the catalog holds
    /// no name.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// Which bucket of attribute_values the name came from. 'sku' means the
    /// catalog holds no name for this product — show that as a missing name, not
    /// as a name.
    #[serde(rename = "source", default)]
    pub source: String,
}
