use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryCommitRequest {
    /// The order whose active reservations are committed (shipment).
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
}
