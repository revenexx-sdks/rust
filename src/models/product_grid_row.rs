use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductGridRow {
    /// The grid cells: one key per attribute code that `columns` lists with
    /// `source: "attribute"`, holding the value already resolved out of
    /// `attribute_values` for the requested context. A code the product carries no
    /// value for is null rather than absent, so a row is the same shape whatever
    /// it holds. The keys are the tenant's own attribute codes, which is why this
    /// object has no fixed properties — read `columns` for the set.
    #[serde(rename = "attributes", default)]
    pub attributes: serde_json::Value,
    /// The stored `products.completeness` document, verbatim. Null means it has
    /// never been computed — not that the product is empty.
    #[serde(rename = "completeness", default)]
    pub completeness: serde_json::Value,
    /// Whether the product is offered.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// That family's code, resolved here so a grid can show and group by it
    /// without a second read.
    #[serde(rename = "family_code", default)]
    pub family_code: String,
    /// The product's family. Null is the state that makes completeness impossible.
    #[serde(rename = "family_id", default)]
    pub family_id: String,
    /// The product's id — what a row click navigates with.
    #[serde(rename = "id", default)]
    pub id: String,
    /// 'simple', 'model' or 'variant' — a model is a row a person should not
    /// price or sell.
    #[serde(rename = "kind", default)]
    pub kind: String,
    /// The resolved display name. Never empty; read `label_source` before showing
    /// it as a name.
    #[serde(rename = "label", default)]
    pub label: String,
    /// Which attribute code the name was read from, per this product's family.
    #[serde(rename = "label_attribute", default)]
    pub label_attribute: String,
    /// Which bucket of attribute_values the name came from. 'sku' means the
    /// catalog holds no name for this product — show that as a missing name, not
    /// as a name.
    #[serde(rename = "label_source", default)]
    pub label_source: String,
    /// The merchant's article number.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// When the product row was last written — the column a "recently changed"
    /// sort uses.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
