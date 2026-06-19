use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryReceiveRequest {
    /// The inbound items (at most 200).
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::InventoryStockItem>,
    /// Receiving location (default 'main').
    #[serde(rename = "location_code", default)]
    pub location_code: String,
    /// Ledger note (e.g. delivery note number).
    #[serde(rename = "reason", default)]
    pub reason: String,
}
