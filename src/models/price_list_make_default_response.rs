use serde::{Deserialize, Serialize};

/// The list as it now stands, plus whoever lost the flag.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceListMakeDefaultResponse {
    /// Codes of the lists that lost the flag — empty when this list already held
    /// it, which is what makes a repeated call free.
    #[serde(rename = "demoted", default)]
    pub demoted: Vec<String>,
    /// A price list: one currency, one tax basis, one validity window, one buyer
    /// scope — and the entries that price items in it. Which list wins for a
    /// given buyer is decided by scope first, then priority, then the default
    /// flag; see prices.resolve.
    #[serde(rename = "price_list", default)]
    pub price_list: crate::models::PriceList,
}
