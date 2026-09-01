use serde::{Deserialize, Serialize};

/// Just enough of the order to render the answer — the full row is GET
/// /orders/{id}.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderShippableOrder {
    /// Whether the order has SHIPPED, and the one dimension nobody writes: it is
    /// DERIVED after every quantity change from the positions' own bookkeeping.
    /// 'fulfilled' means shipped >= ordered − cancelled across all positions,
    /// 'partial' means something went out. Sending it has no effect; ship, cancel
    /// or return something and it moves.
    #[serde(rename = "fulfillment_status", default)]
    pub fulfillment_status: String,
    /// Why the order is held, in the words the shipping guard quotes back. Null
    /// when it is not held — releasing a hold clears it.
    #[serde(rename = "hold_reason", default)]
    pub hold_reason: String,
    /// The order this answer is about.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The order number a human quotes — drawn from the tenant's order range at
    /// place-time, unique per tenant and never reused. It is NOT the id: every
    /// route addresses an order by uuid, and GET /orders?number=… is how a
    /// number becomes one.
    #[serde(rename = "number", default)]
    pub number: String,
    /// A business stop, ORTHOGONAL to status: a held order keeps its lifecycle
    /// state and is refused at the guards. How far the hold reaches is the
    /// tenant's call (on_hold_blocks: shipping only, shipping and cancellation, or
    /// nothing at all).
    #[serde(rename = "on_hold", default)]
    pub on_hold: bool,
    /// Where the order stands in its LIFECYCLE, and one of three independent
    /// status dimensions. 'pending' = created but not placed, an order waiting for
    /// approval; 'placed' = accepted, nothing shipped; 'in_fulfillment' = part of
    /// it has gone out, or all of it has and the tenant does not close on
    /// shipment; 'completed' and 'cancelled' end it. Moved by the action routes
    /// only — it is not writable through PUT /orders/{id}.
    #[serde(rename = "status", default)]
    pub status: String,
}
