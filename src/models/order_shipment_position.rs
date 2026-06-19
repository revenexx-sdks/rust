use serde::{Deserialize, Serialize};

/// A position quantity to ship — guarded against the open quantity.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderShipmentPosition {
    /// The order item (position) to act on.
    #[serde(rename = "order_item_id", default)]
    pub order_item_id: String,
    /// Defaults to the full remaining quantity of the position.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
}
