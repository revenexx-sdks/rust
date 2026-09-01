use serde::{Deserialize, Serialize};

/// A position quantity to cancel — guarded against the open (unshipped,
/// uncancelled) quantity.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderCancelPosition {
    /// The order item (position) to act on. Read the ids from GET /orders/{id}
    /// (items[].id) or GET /orders/{id}/shippable (positions[].order_item_id) —
    /// an id this order does not carry is a 400.
    #[serde(rename = "order_item_id", default)]
    pub order_item_id: String,
    /// Defaults to the full remaining quantity of the position.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
}
