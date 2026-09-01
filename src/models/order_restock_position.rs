use serde::{Deserialize, Serialize};

/// One quantity to put back into stock, named the way the inventories app
/// wants it: by product, by sku, and how much.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderRestockPosition {
    /// The catalog product to restock. Null on a custom line, which is why `sku`
    /// is carried alongside it.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// How much came back on this position, in the position's own unit.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// The article number to restock — the key a warehouse actually books
    /// against.
    #[serde(rename = "sku", default)]
    pub sku: String,
}
