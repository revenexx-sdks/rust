use serde::{Deserialize, Serialize};

/// One line of a delivery note: how much of one order position went out in one
/// shipment.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderShipmentItem {
    /// When the booking was written.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Primary key of the booked position line.
    #[serde(rename = "id", default)]
    pub id: String,
    /// Which order position went out. Always a position of the same order as the
    /// shipment.
    #[serde(rename = "order_item_id", default)]
    pub order_item_id: String,
    /// How much of that position this shipment carried. The sum of these over all
    /// shipments is the position's quantity_shipped.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// The shipment this booking belongs to. Deleting the shipment deletes it.
    #[serde(rename = "shipment_id", default)]
    pub shipment_id: String,
}
