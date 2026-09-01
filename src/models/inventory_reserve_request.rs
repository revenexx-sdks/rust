use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryReserveRequest {
    /// When this hold lapses. The sweeper — POST
    /// /inventories/reservations/sweep, and the 'expire-reservations' schedule
    /// that runs it every 15 minutes — releases everything past this moment
    /// exactly as a cancellation would, so an abandoned checkout stops holding
    /// stock on its own. Null means the row named no deadline: it is swept on its
    /// AGE instead once `reservation_ttl_minutes` is above 0, which is what makes
    /// turning that setting on retroactive. Omit it to let the
    /// `reservation_ttl_minutes` setting stamp one (0 — its default — means no
    /// deadline at all); send one to hold this order for a window of its own, e.g.
    /// a quote that stands until Friday.
    #[serde(rename = "expires_at", default)]
    pub expires_at: String,
    /// The items to hold, at most 200 in one call — a whole cart in one request.
    /// The call is planned before anything is written, so either every item is
    /// placed or nothing is.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::InventoryStockItem>,
    /// Where a BACKORDERED item is booked when no location holds a stock row for
    /// it at all — the last fallback, not the allocator: which location serves
    /// an item that IS in stock comes from `allocation_strategy`. Omitted, the
    /// `default_location_code` setting decides.
    #[serde(rename = "location_code", default)]
    pub location_code: String,
    /// The order this hold belongs to. The caller supplies it — this app mints
    /// nothing — and it is the handle POST /inventories/release and POST
    /// /inventories/commit act on, so it has to be the same string the order
    /// carries elsewhere. At least one character (CHECK `length(order_ref) > 0`).
    /// Not unique: an order holds one reservation per item, and they are released
    /// or committed together. Reserving twice under the same reference ADDS holds
    /// rather than replacing them — release first if you mean to replace.
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
    /// Inline single-item form: the product to move, instead of a one-entry
    /// `items` array. The two forms are equivalent — nothing downstream knows
    /// which arrived.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Inline single-item form: how many to hold. Positive — the hold is
    /// expressed as a positive reservation, while the ledger booking it writes
    /// carries the negative.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// Where the order is going. Read ONLY when the tenant's `allocation_strategy`
    /// is 'nearest' — under 'priority' or 'single_location' it is accepted and
    /// ignored, so sending it is never wrong, it is just not always heard.
    #[serde(rename = "ship_to", default)]
    pub ship_to: crate::models::InventoryShipTo,
    /// Inline single-item form: the article number to move (instead of
    /// `product_id`).
    #[serde(rename = "sku", default)]
    pub sku: String,
}
