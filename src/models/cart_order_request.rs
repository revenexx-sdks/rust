use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartOrderRequest {
    /// The order number this cart becomes, in order management's own numbering.
    /// Stored on the cart — filtering on it is how anyone gets from an order
    /// back to the cart behind it — and it is also the reference the stock
    /// reservation is booked under. Omit it and the cart id is used for the
    /// reservation instead.
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
}
