use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryAvailabilityRequest {
    /// The items to check, at most 200 in one call. A cart, a category page, a
    /// feed row — one call answers them all, which is why this route is the
    /// batch one.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::InventoryAvailabilityItem>,
    /// Restrict the check to ONE location, by its code — the stock a
    /// click-and-collect store can promise today. Omitted, every ENABLED location
    /// is summed; a disabled one is never counted either way.
    #[serde(rename = "location_code", default)]
    pub location_code: String,
    /// Inline single-item form: the product to move, instead of a one-entry
    /// `items` array. The two forms are equivalent — nothing downstream knows
    /// which arrived.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Inline single-item form: how many are wanted (default 1). It decides
    /// `orderable` and nothing else.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// Inline single-item form: the article number to move (instead of
    /// `product_id`).
    #[serde(rename = "sku", default)]
    pub sku: String,
}
