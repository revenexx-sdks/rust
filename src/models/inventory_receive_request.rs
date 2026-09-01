use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryReceiveRequest {
    /// The goods that arrived, at most 200 in one call — a delivery, a
    /// production batch, an opening balance.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::InventoryStockItem>,
    /// Which location took the delivery. Omitted, the `default_location_code`
    /// setting decides; a code no location carries is answered 400 rather than
    /// booked somewhere else.
    #[serde(rename = "location_code", default)]
    pub location_code: String,
    /// Inline single-item form: the product to move, instead of a one-entry
    /// `items` array. The two forms are equivalent — nothing downstream knows
    /// which arrived.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Inline single-item form: how many arrived. Positive.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// What the ledger should record about this receipt — a delivery note
    /// number, a production order. Owed only when `movement_reason_required` is
    /// 'all'; the contract does not require it, because whether it is owed is the
    /// tenant's setting and not this route's rule.
    #[serde(rename = "reason", default)]
    pub reason: String,
    /// Inline single-item form: the article number to move (instead of
    /// `product_id`).
    #[serde(rename = "sku", default)]
    pub sku: String,
}
