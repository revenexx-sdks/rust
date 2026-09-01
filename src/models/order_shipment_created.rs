use serde::{Deserialize, Serialize};

/// What the booking produced: the new shipment with the quantities it took,
/// and the order as it now stands.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderShipmentCreated {
    /// The order after the booking: fulfillment_status is re-derived from the
    /// positions, and status may have moved to in_fulfillment or (depending on the
    /// tenant's auto_complete_on) completed.
    #[serde(rename = "order", default)]
    pub order: crate::models::Order,
    /// The shipment that was created, WITH the position quantities it booked —
    /// the only place a caller learns which quantities actually went out when the
    /// positions were defaulted.
    #[serde(rename = "shipment", default)]
    pub shipment: crate::models::OrderShipment,
}
