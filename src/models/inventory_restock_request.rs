use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryRestockRequest {
    /// The returned items (at most 200).
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::InventoryStockItem>,
    /// Restocking location (default 'main').
    #[serde(rename = "location_code", default)]
    pub location_code: String,
    /// Originating order (ledger reference).
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
    /// Ledger note (e.g. return reason).
    #[serde(rename = "reason", default)]
    pub reason: String,
}
