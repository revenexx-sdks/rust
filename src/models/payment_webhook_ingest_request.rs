use serde::{Deserialize, Serialize};

/// The dispatch envelope from webhooks.revenexx.com — request.body carries
/// the raw, vendor-shaped PSP callback (stripe payment intents or the generic
/// {event, psp_payment_id?, order_ref?, error?} shape). Intentionally
/// unconstrained so no PSP notification is ever rejected at the gate.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentWebhookIngestRequest {
}
