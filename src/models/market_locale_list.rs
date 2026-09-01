use serde::{Deserialize, Serialize};

/// One page of locales of a market, the page it sits on, and the filters that
/// produced it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketLocaleList {
    /// The exact-column filters this call applied, echoed back. Every value is the
    /// raw query string, never the column's own type: `?is_default=true` comes
    /// back as `"true"`. A `?column=value` naming a column this entity does not
    /// have is DROPPED rather than refused — the call answers 200 with the
    /// unfiltered list, and the key missing from here is the only way to find out.
    #[serde(rename = "filter", default)]
    pub filter: crate::models::MarketLocaleFilter,
    /// The locales of a market on this page, in `order` — by `position`
    /// ascending unless the call asked otherwise.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::MarketLocale>,
    /// Where in the result set this answer sits. `limit` and `offset` are the
    /// values that were APPLIED, not the ones that were asked for — the data
    /// plane clamps rather than refuses, so an out-of-range or unparseable value
    /// comes back corrected here instead of as a 400.
    #[serde(rename = "page", default)]
    pub page: crate::models::MarketsPage,
}
