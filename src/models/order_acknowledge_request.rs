use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderAcknowledgeRequest {
    /// The fulfilling system's order reference (e.g. the ERP order number).
    #[serde(rename = "external_ref", default)]
    pub external_ref: String,
}
