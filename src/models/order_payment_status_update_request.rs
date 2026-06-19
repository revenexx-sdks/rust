use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderPaymentStatusUpdateRequest {
    /// Reference into the payment system — merged into the order's payment
    /// snapshot.
    #[serde(rename = "payment_id", default)]
    pub payment_id: String,
    /// The new payment dimension value.
    #[serde(rename = "status", default)]
    pub status: String,
}
