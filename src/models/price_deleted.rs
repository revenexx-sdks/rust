use serde::{Deserialize, Serialize};

/// The row is gone. Deleting a price list cascades to its entries.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceDeleted {
    /// Always true — a row that was not there answers 404 instead.
    #[serde(rename = "deleted", default)]
    pub deleted: bool,
    /// The row that was removed.
    #[serde(rename = "id", default)]
    pub id: String,
}
