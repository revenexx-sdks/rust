use serde::{Deserialize, Serialize};

/// Create a shipment. Omitted positions = ship everything still open.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderShipmentCreateRequest {
    #[serde(rename = "carrier", default)]
    pub carrier: String,
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Delivery note number — drawn from the 'delivery' range when omitted.
    #[serde(rename = "number", default)]
    pub number: String,
    /// Omitted = every position with open quantity, in full.
    #[serde(rename = "positions", default)]
    pub positions: Vec<crate::models::OrderShipmentPosition>,
    /// Defaults to now.
    #[serde(rename = "shipped_at", default)]
    pub shipped_at: String,
    #[serde(rename = "tracking_code", default)]
    pub tracking_code: String,
    #[serde(rename = "tracking_url", default)]
    pub tracking_url: String,
}
