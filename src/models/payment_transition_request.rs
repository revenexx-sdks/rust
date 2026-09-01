use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentTransitionRequest {
    /// The operator's own words for why. Kept on the payment
    /// (`metadata.cancel_reason` / `metadata.refund_reason`) AND handed to the
    /// provider's own cancellation or refund reason field, so it is readable in
    /// the PSP's dashboard too. Trimmed and cut at 500 characters.
    #[serde(rename = "reason", default)]
    pub reason: String,
}
