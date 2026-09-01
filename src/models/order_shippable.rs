use serde::{Deserialize, Serialize};

/// What a shipment of this order may still contain, and whether one would be
/// accepted at all — answered by the same code POST /orders/{id}/ship runs,
/// so the two cannot drift.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderShippable {
    /// Why not, in the very words POST /orders/{id}/ship would refuse with —
    /// including the hold reason where there is one. Null when `shippable` is
    /// true.
    #[serde(rename = "blocked_reason", default)]
    pub blocked_reason: String,
    /// How many positions still have an open quantity — the number of lines a
    /// shipment dialog would offer.
    #[serde(rename = "open_positions", default)]
    pub open_positions: i64,
    /// The summed open quantity over those positions. Mixes units where the order
    /// does, so it is a headline figure, not a total to act on.
    #[serde(rename = "open_quantity", default)]
    pub open_quantity: f64,
    /// Just enough of the order to render the answer — the full row is GET
    /// /orders/{id}.
    #[serde(rename = "order", default)]
    pub order: crate::models::OrderShippableOrder,
    /// Every position of the order, in position order, each with its open
    /// quantity.
    #[serde(rename = "positions", default)]
    pub positions: Vec<crate::models::OrderShippablePosition>,
    /// Whether a shipment would be accepted RIGHT NOW — the one question a
    /// "create shipment" button should be enabled on. False when the order is
    /// held, cancelled, completed, or has nothing open.
    #[serde(rename = "shippable", default)]
    pub shippable: bool,
}
