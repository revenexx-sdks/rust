use serde::{Deserialize, Serialize};

/// The strings to translate. They are forwarded to the tenant's provider
/// verbatim.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageTranslateRequest {
    /// The strings to translate. This app reads no element of the list — the
    /// provider defines the contract, and the blökkli adapter sends the fields
    /// below.
    #[serde(rename = "items", default)]
    pub items: Vec<serde_json::Value>,
}
