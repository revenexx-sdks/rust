use serde::{Deserialize, Serialize};

/// One item and its SIGNED correction: 'product_id' or 'sku', plus a non-zero
/// delta.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryAdjustItem {
    /// The product to move, as the products app knows it. Give this OR `sku` —
    /// an item that names neither is answered 400. Matching is exact: a stock row
    /// keyed by SKU is not found by product id.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// The SIGNED correction to `on_hand`: −3 writes off three, +3 finds three.
    /// It is a delta, not the new balance. Zero is refused (400) because a
    /// correction of nothing is a mistake, not a booking — the rule is the
    /// handler's, not a database CHECK, which is why it is stated here rather than
    /// declared as a bound.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// The article number to move, when the item has no product id. Give this OR
    /// `product_id`.
    #[serde(rename = "sku", default)]
    pub sku: String,
}
