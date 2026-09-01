use serde::{Deserialize, Serialize};

/// The dispatch envelope from webhooks.revenexx.com. Nothing is required and
/// nothing is constrained — three keys are read, and the rest is carried
/// along.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentWebhookIngestRequest {
    /// The dispatcher's delivery id. Echoed back as `delivery_id` so a delivery
    /// and what the ledger did can be correlated.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The captured HTTP request as the PSP sent it.
    #[serde(rename = "request", default)]
    pub request: String,
    /// Whether the ingress verified the callback signature against the provider's
    /// `webhook_secret`. An explicit false is refused with 422: an endpoint may
    /// run in annotate mode, and the ledger stays sovereign over one that does.
    #[serde(rename = "verified", default)]
    pub verified: String,
}
