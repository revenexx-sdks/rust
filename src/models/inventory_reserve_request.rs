use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryReserveRequest {
    /// Optional reservation expiry.
    #[serde(rename = "expires_at", default)]
    pub expires_at: String,
    /// The items to reserve — all-or-nothing (at most 200).
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::InventoryStockItem>,
    /// The order this reservation belongs to.
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
}
