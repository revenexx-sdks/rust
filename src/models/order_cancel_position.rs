use serde::{Deserialize, Serialize};

/// A position quantity to cancel — guarded against the open (unshipped,
/// uncancelled) quantity.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderCancelPosition {
    /// The order item (position) to act on.
    #[serde(rename = "order_item_id", default)]
    pub order_item_id: String,
    /// Defaults to the full remaining quantity of the position.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
}
