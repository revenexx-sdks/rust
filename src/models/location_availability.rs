use serde::{Deserialize, Serialize};

/// What one location holds of this item. Only enabled locations appear, and
/// only those with a stock row for the item — a location that has never held
/// it is absent rather than zero.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocationAvailability {
    /// on_hand − reserved at this location — what this one place can still
    /// promise.
    #[serde(rename = "available", default)]
    pub available: f64,
    /// The location CODE (`locations.code`) — the same value `location_code`
    /// takes in a request. Falls back to the raw location id in the rare case
    /// where the location row disappeared between the two reads.
    #[serde(rename = "location", default)]
    pub location: String,
    /// Physically at this location, promised units included.
    #[serde(rename = "on_hand", default)]
    pub on_hand: f64,
    /// Held for orders at this location.
    #[serde(rename = "reserved", default)]
    pub reserved: f64,
}
