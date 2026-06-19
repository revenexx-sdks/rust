use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryReleaseRequest {
    /// The order whose active reservations are released.
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
}
