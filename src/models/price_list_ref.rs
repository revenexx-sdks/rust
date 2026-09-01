use serde::{Deserialize, Serialize};

/// The price list this answer came out of — enough to link to it or to
/// explain the number to a merchant ("this came from the dealer list").
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceListRef {
    /// The list’s unique per-tenant code.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The list, by id — the same id `GET /prices/lists/{id}` takes.
    #[serde(rename = "id", default)]
    pub id: String,
}
