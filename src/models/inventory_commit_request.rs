use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryCommitRequest {
    /// The order this hold belongs to. The caller supplies it — this app mints
    /// nothing — and it is the handle POST /inventories/release and POST
    /// /inventories/commit act on, so it has to be the same string the order
    /// carries elsewhere. At least one character (CHECK `length(order_ref) > 0`).
    /// Not unique: an order holds one reservation per item, and they are released
    /// or committed together. Every ACTIVE hold under this reference ships:
    /// `on_hand` and `reserved` both fall and a `shipment` booking is written for
    /// each. Unlike release, committing an order that has nothing active is a 422
    /// — it means the hold was already released or already shipped, and shipping
    /// twice is worth saying out loud.
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
}
