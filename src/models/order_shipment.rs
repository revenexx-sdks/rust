use serde::{Deserialize, Serialize};

/// One handover to a carrier — a delivery note. An order has as many of
/// these as it took to get the goods out; each carries the position quantities
/// it booked.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderShipment {
    /// Who is carrying it, in the merchant's own words. Free text — this app
    /// neither validates it nor knows the carrier's API.
    #[serde(rename = "carrier", default)]
    pub carrier: String,
    /// When the shipment was booked here, which is not necessarily when it left
    /// — that is shipped_at.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Primary key of the shipment.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The booked position quantities of this shipment.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::OrderShipmentItem>,
    /// Free-form data for the caller — the warehouse system's own reference for
    /// this handover. Stored and returned untouched.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// The DELIVERY NOTE number — drawn from the tenant's delivery range, unique
    /// per tenant, and a different series from the order number. A caller may
    /// supply its own when the number is issued by the warehouse system instead.
    #[serde(rename = "number", default)]
    pub number: String,
    /// The order this shipment belongs to. Deleting the order deletes its
    /// shipments.
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    /// When the goods actually left. Defaults to now, and a caller may backdate it
    /// — a shipment booked on Monday for a Friday handover says Friday.
    #[serde(rename = "shipped_at", default)]
    pub shipped_at: String,
    /// The consignment number the carrier issued. Free text: every carrier formats
    /// it differently and this app stores whatever it is given.
    #[serde(rename = "tracking_code", default)]
    pub tracking_code: String,
    /// Where a human can follow the parcel. Supplied by the caller — this app
    /// does not build it, because only the caller knows the carrier's tracking
    /// address.
    #[serde(rename = "tracking_url", default)]
    pub tracking_url: String,
}
