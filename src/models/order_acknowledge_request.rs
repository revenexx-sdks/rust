use serde::{Deserialize, Serialize};

/// The acknowledgement carries one field, and it is optional: sending {} still
/// stamps acknowledged_at, which is the point of the call. acknowledged_at is
/// the server's clock and is never taken from the body.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderAcknowledgeRequest {
    /// The FULFILLING system's reference for this order, typically the ERP order
    /// number. Written once by POST /orders/{id}/acknowledge and null until an
    /// integration acknowledged it. Keeps the existing value when omitted.
    #[serde(rename = "external_ref", default)]
    pub external_ref: String,
}
