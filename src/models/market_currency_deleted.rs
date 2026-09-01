use serde::{Deserialize, Serialize};

/// Confirmation that the currency of a market is gone. The row itself is not
/// returned — read it before deleting if you need it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketCurrencyDeleted {
    /// Always true — a row that was not there is a 404, not a false.
    #[serde(rename = "deleted", default)]
    pub deleted: bool,
    /// The id of the row that was deleted.
    #[serde(rename = "id", default)]
    pub id: String,
}
