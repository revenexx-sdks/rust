use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryRestockRequest {
    /// The goods that came back, at most 200 in one call. Whether they rejoin
    /// sellable stock is `restock`, not this list.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::InventoryStockItem>,
    /// Where the goods came back to — a returns warehouse is a location like any
    /// other. Omitted, the `default_location_code` setting decides.
    #[serde(rename = "location_code", default)]
    pub location_code: String,
    /// The order the goods came back from. It is written onto the ledger booking,
    /// so the return shows up in that order's stock history next to its reserve
    /// and shipment — no reservation is touched by it.
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
    /// Inline single-item form: the product to move, instead of a one-entry
    /// `items` array. The two forms are equivalent — nothing downstream knows
    /// which arrived.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Inline single-item form: how many came back. Positive.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// Why the goods came back — 'wrong size', 'damaged on arrival'. Owed only
    /// when `movement_reason_required` is 'all'.
    #[serde(rename = "reason", default)]
    pub reason: String,
    /// Do these goods rejoin SELLABLE stock? A merchant decision, not a fact:
    /// apparel usually restocks, hygiene articles never do, many merchants inspect
    /// first. Omit it to follow the `restock_on_return_default` setting. `false`
    /// answers `restocked: false`, moves nothing and books NOTHING — there is no
    /// movement to write, because no stock moved, and that is the branch that
    /// makes this route a 200 while its sibling `receive` is a 201.
    #[serde(rename = "restock", default)]
    pub restock: bool,
    /// Inline single-item form: the article number to move (instead of
    /// `product_id`).
    #[serde(rename = "sku", default)]
    pub sku: String,
}
