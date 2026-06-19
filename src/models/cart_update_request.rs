use serde::{Deserialize, Serialize};

/// Only safe columns are updatable — status moves through the lifecycle
/// routes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartUpdateRequest {
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    /// ISO 4217 code.
    #[serde(rename = "currency", default)]
    pub currency: String,
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "name", default)]
    pub name: String,
}
