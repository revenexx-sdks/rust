use serde::{Deserialize, Serialize};

/// The policy this answer was computed under — the tenant settings in force
/// plus where the currency came from.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceResolveBasis {
    /// false ⇒ a buyer with no contact/organization is answered on_request for
    /// everything.
    #[serde(rename = "anonymous_resolve_allowed", default)]
    pub anonymous_resolve_allowed: bool,
    /// Where `currency` came from: the request, the buyer market's own currency,
    /// the tenant's default_currency setting, or the shipped fallback.
    #[serde(rename = "currency_source", default)]
    pub currency_source: String,
    /// The instant validity windows were evaluated at.
    #[serde(rename = "evaluated_at", default)]
    pub evaluated_at: String,
    /// Which list won where specificity and priority tied.
    #[serde(rename = "price_list_priority_tiebreak", default)]
    pub price_list_priority_tiebreak: String,
    /// Decimals every DERIVED amount (net, gross, line totals) was rounded to.
    #[serde(rename = "price_precision", default)]
    pub price_precision: i64,
    /// How those amounts landed on the last decimal.
    #[serde(rename = "rounding_mode", default)]
    pub rounding_mode: String,
    /// Tenant setting: the basis a price list that states none is read on.
    #[serde(rename = "tax_inclusive_default", default)]
    pub tax_inclusive_default: String,
}
