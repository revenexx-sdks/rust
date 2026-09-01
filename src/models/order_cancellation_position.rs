use serde::{Deserialize, Serialize};

/// One position quantity this cancellation removed.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderCancellationPosition {
    /// The order item this quantity was booked against — an id out of the same
    /// order, never another one.
    #[serde(rename = "order_item_id", default)]
    pub order_item_id: String,
    /// The quantity booked on that position, in the position's own unit. Three
    /// decimal places, so 0.5 m of cable is a real booking.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
}
