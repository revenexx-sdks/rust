use serde::{Deserialize, Serialize};

/// One position quantity registered for return.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderReturnedPosition {
    /// The order item this quantity was booked against — an id out of the same
    /// order, never another one.
    #[serde(rename = "order_item_id", default)]
    pub order_item_id: String,
    /// The quantity booked on that position, in the position's own unit. Three
    /// decimal places, so 0.5 m of cable is a real booking.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// Whether this quantity is reported for restocking when the return completes.
    /// Restocking itself stays an explicit inventories.restock call by the
    /// orchestrator — this app never writes another app's stock.
    #[serde(rename = "restock", default)]
    pub restock: bool,
}
