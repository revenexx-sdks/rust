use serde::{Deserialize, Serialize};

/// One item to check: 'product_id' or 'sku'. Checking is free of consequence
/// — it books nothing and holds nothing.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryAvailabilityItem {
    /// The product to move, as the products app knows it. Give this OR `sku` —
    /// an item that names neither is answered 400. Matching is exact: a stock row
    /// keyed by SKU is not found by product id.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// How many are wanted. It only decides `orderable`; the on_hand / reserved /
    /// available figures come back whatever it is. Omit it (or send null) to ask
    /// "is this sellable at all?", which is a check against 1.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// The article number to move, when the item has no product id. Give this OR
    /// `product_id`.
    #[serde(rename = "sku", default)]
    pub sku: String,
}
