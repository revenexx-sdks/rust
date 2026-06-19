use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryAvailabilityRequest {
    /// The items to check (batch, at most 200).
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::InventoryAvailabilityItem>,
    /// Restrict the check to one location (default: all enabled locations).
    #[serde(rename = "location_code", default)]
    pub location_code: String,
}
