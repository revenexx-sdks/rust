use serde::{Deserialize, Serialize};

/// One answer per requested item, in request order, plus the currency, the tax
/// context and the policy the numbers were computed under.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceResolveResponse {
    /// The policy this answer was computed under — the tenant settings in force
    /// plus where the currency came from.
    #[serde(rename = "basis", default)]
    pub basis: crate::models::PriceResolveBasis,
    /// ISO 4217 currency the whole answer is quoted in, and the currency lists had
    /// to match to be candidates at all. `basis.currency_source` says where it
    /// came from: the request, the buyer market, the tenant setting, or the
    /// shipped fallback.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// One entry per requested item, in the order the items were sent. An item
    /// that could not be priced is present and `on_request`, never missing.
    #[serde(rename = "prices", default)]
    pub prices: Vec<crate::models::ResolvedPrice>,
    /// Tax resolution status of this answer. resolved=false ⇒ tax_class/tax_rate
    /// are unknown, NOT zero.
    #[serde(rename = "tax", default)]
    pub tax: crate::models::PriceTaxContext,
}
