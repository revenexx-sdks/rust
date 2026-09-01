use serde::{Deserialize, Serialize};

/// The path id is the SOURCE market (a uuid or a market code). Everything the
/// new market does not inherit is here. The copy flags default to true;
/// `is_default` is never copied, and the new market always gets its own base
/// currency registered and marked default.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketCloneRequest {
    /// Code of the NEW market (unique per tenant).
    #[serde(rename = "code", default)]
    pub code: String,
    /// Copy the source's traded currencies. Default true. The new market's own
    /// base currency is registered and marked default either way.
    #[serde(rename = "copy_currencies", default)]
    pub copy_currencies: bool,
    /// Copy the source's locales. Default true. False leaves the new market with
    /// no language of its own, so the tenant fallback_locale is seeded instead —
    /// it is never left with none.
    #[serde(rename = "copy_locales", default)]
    pub copy_locales: bool,
    /// Copy the source's tax classes, rates and all. Default true. False leaves
    /// the market unable to tax anything, which readiness reports as blocking.
    #[serde(rename = "copy_tax_classes", default)]
    pub copy_tax_classes: bool,
    /// Base currency of the new market (ISO 4217). Defaults to the source
    /// market's, and is registered and marked default on the new one either way.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// Display name of the new market. Defaults to its code.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Status of the new market. Defaults to 'active'; clone it 'inactive' to
    /// build it out before it serves anyone.
    #[serde(rename = "status", default)]
    pub status: String,
}
