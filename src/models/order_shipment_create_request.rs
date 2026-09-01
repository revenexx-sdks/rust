use serde::{Deserialize, Serialize};

/// Book what went out. Every field is optional: an empty body ships every
/// position that still has an open quantity, in full, on a delivery note
/// number drawn from the tenant's delivery range — which is the whole
/// payload for the common case.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderShipmentCreateRequest {
    /// Who is carrying it, in the merchant's own words. Free text — this app
    /// neither validates it nor knows the carrier's API.
    #[serde(rename = "carrier", default)]
    pub carrier: String,
    /// Free-form data for the caller — the warehouse system's own reference for
    /// this handover. Stored and returned untouched.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// The DELIVERY NOTE number — drawn from the tenant's delivery range, unique
    /// per tenant, and a different series from the order number. A caller may
    /// supply its own when the number is issued by the warehouse system instead.
    /// Drawn from the 'delivery' range when omitted; supply one only when the
    /// number is issued elsewhere.
    #[serde(rename = "number", default)]
    pub number: String,
    /// What this shipment carries. Omitted = every position with an open quantity,
    /// in full. GET /orders/{id}/shippable answers exactly the budget each one is
    /// guarded against.
    #[serde(rename = "positions", default)]
    pub positions: Vec<crate::models::OrderShipmentPosition>,
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
