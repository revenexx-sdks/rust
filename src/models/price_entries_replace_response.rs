use serde::{Deserialize, Serialize};

/// The list as it now stands: everything that was there is gone and these are
/// the rows that took its place.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceEntriesReplaceResponse {
    /// The complete new entry set, as stored — including the ids and timestamps
    /// the database filled in.
    #[serde(rename = "entries", default)]
    pub entries: Vec<crate::models::PriceEntry>,
}
