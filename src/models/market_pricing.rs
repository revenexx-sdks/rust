use serde::{Deserialize, Serialize};

/// Whether a stored price in this market is NET or GROSS — the market layer
/// of an answer the prices app also holds. A price list's own tax_basis wins
/// over this; `tax_basis: null` with `source: 'unset'` means this market
/// declares nothing and the reader must fall through to the tenant's own
/// default.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketPricing {
    /// The raw `prices_include_tax` setting resolved for this market. Null means
    /// the market declares nothing — it is NOT a false, and turning it into one
    /// is the bug this key exists to prevent.
    #[serde(rename = "prices_include_tax", default)]
    pub prices_include_tax: bool,
    /// Where the value came from. 'market' — configured on this market. 'tenant'
    /// — the market holds no value of its own and the tenant baseline answered.
    /// 'unset' — nothing is configured anywhere in this app, and the reader must
    /// fall through to the prices app's tax_inclusive_default.
    #[serde(rename = "source", default)]
    pub source: String,
    /// The same answer in the prices app's own vocabulary, so the two halves of
    /// the platform use one word: 'gross' means a stored price already contains
    /// tax, 'net' means tax is added on top. Null means fall through to the
    /// tenant's own default.
    #[serde(rename = "tax_basis", default)]
    pub tax_basis: String,
}
