use serde::{Deserialize, Serialize};

/// Where the order is going. Read ONLY when the tenant's `allocation_strategy`
/// is 'nearest' — under 'priority' or 'single_location' it is accepted and
/// ignored, so sending it is never wrong, it is just not always heard.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryShipTo {
    /// ISO country code of the delivery address. Locations whose `address.country`
    /// matches are tried before the rest, which is what stops a German order
    /// pulling from an overseas warehouse that merely sorts first.
    #[serde(rename = "country", default)]
    pub country: String,
    /// Prefer this location above everything else — a click-and-collect store
    /// the customer picked. It is a preference, not a demand: if it cannot cover
    /// the item the allocator moves on to the next location.
    #[serde(rename = "location_code", default)]
    pub location_code: String,
}
