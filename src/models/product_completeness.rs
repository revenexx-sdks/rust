use serde::{Deserialize, Serialize};

/// What was measured and stored into `products.completeness` by this call —
/// how much of what the family requires the product actually carries.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductCompleteness {
    /// When this measurement was taken. It is a snapshot: editing the product does
    /// not update it, the next `POST /products/{id}/completeness` does.
    #[serde(rename = "computed_at", default)]
    pub computed_at: String,
    /// How many of those carry a value — in ANY bucket, so a name held only in
    /// German counts.
    #[serde(rename = "filled", default)]
    pub filled: i64,
    /// Attribute codes with no value in any bucket.
    #[serde(rename = "missing", default)]
    pub missing: Vec<String>,
    /// filled / required, 0..1. A family that requires nothing is 1, not
    /// undefined.
    #[serde(rename = "ratio", default)]
    pub ratio: f64,
    /// Attributes the product's family marks is_required.
    #[serde(rename = "required", default)]
    pub required: i64,
}
