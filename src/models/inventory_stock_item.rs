use serde::{Deserialize, Serialize};

/// One item and how much of it: 'product_id' or 'sku', plus a positive
/// quantity.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryStockItem {
    /// The product to move, as the products app knows it. Give this OR `sku` —
    /// an item that names neither is answered 400. Matching is exact: a stock row
    /// keyed by SKU is not found by product id.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// How many units this booking moves. Always POSITIVE here — the direction
    /// is the route (receive adds, reserve holds, restock returns), not the sign.
    /// Zero or a negative number is answered 400; a signed correction is what POST
    /// /inventories/adjust is for.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// The article number to move, when the item has no product id. Give this OR
    /// `product_id`.
    #[serde(rename = "sku", default)]
    pub sku: String,
}
