use serde::{Deserialize, Serialize};

/// Confirmation that the tax class of a market is gone. The row itself is not
/// returned — read it before deleting if you need it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketTaxClassDeleted {
    /// Always true — a row that was not there is a 404, not a false.
    #[serde(rename = "deleted", default)]
    pub deleted: bool,
    /// The id of the row that was deleted.
    #[serde(rename = "id", default)]
    pub id: String,
    /// False when the cross-app usage question could not be asked (shipping not
    /// installed, or unreachable) — the row was deleted without that guarantee.
    #[serde(rename = "usage_checked", default)]
    pub usage_checked: bool,
}
