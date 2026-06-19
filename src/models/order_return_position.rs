use serde::{Deserialize, Serialize};

/// A position quantity to return — guarded against the shipped (not yet
/// returned) quantity.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderReturnPosition {
    /// The order item (position) to act on.
    #[serde(rename = "order_item_id", default)]
    pub order_item_id: String,
    /// Defaults to the full remaining quantity of the position.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// Report this position for restocking when the return completes (the explicit
    /// inventories.restock call stays with the orchestrator).
    #[serde(rename = "restock", default)]
    pub restock: bool,
}
