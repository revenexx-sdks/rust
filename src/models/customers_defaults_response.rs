use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomersDefaultsResponse {
    /// One entry per value set, keyed by its route name — `payment-terms`,
    /// `address-types`, `lifecycle-stages`, `contact-event-kinds`. Each says what
    /// THIS call did: `created` are the codes it inserted, `existing` the seeded
    /// codes it found already there and left completely alone (a merchant's rename
    /// included). A second call therefore answers with everything under `existing`
    /// and nothing under `created`.
    #[serde(rename = "sets", default)]
    pub sets: serde_json::Value,
}
