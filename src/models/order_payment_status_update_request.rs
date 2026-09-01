use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderPaymentStatusUpdateRequest {
    /// The reference into the payment system. MERGED into the order's payment
    /// snapshot under 'payment_id' — the rest of the snapshot is left alone —
    /// and carried in the order.payment_status.changed event. Omitted leaves the
    /// snapshot untouched.
    #[serde(rename = "payment_id", default)]
    pub payment_id: String,
    /// The new value of the payment dimension. Whether the order is PAID, and the
    /// dimension this app does not decide: it is fed from outside through POST
    /// /orders/{id}/payment-status (the payments app or an ERP), and only seeded
    /// at place-time from payment.status. Orthogonal to the lifecycle — a
    /// completed order can still be open, and a paid one can still be pending.
    #[serde(rename = "status", default)]
    pub status: String,
}
