use serde::{Deserialize, Serialize};

/// The whole of one market: the row, its three collections, and the four
/// resolved answers a client would otherwise have to work out for itself.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketContext {
    /// Every currency this market trades in, in position order. Capped at 200. The
    /// market's own base currency should be among them; readiness reports it as
    /// blocking when it is not.
    #[serde(rename = "currencies", default)]
    pub currencies: Vec<crate::models::MarketCurrency>,
    /// The locale a storefront should render this market in. `source` names where
    /// it came from: 'market' (a locale flagged is_default), 'market_first' (no
    /// flag — first by position) or 'tenant_fallback' (the market registers
    /// none; the tenant's fallback_locale setting answered).
    #[serde(rename = "default_locale", default)]
    pub default_locale: crate::models::MarketDefaultLocale,
    /// How this tenant keys its translations, resolved rather than named: the key
    /// a client WRITES and the order it READS, per locale. Emitting the resolved
    /// answer is the point — a client handed only the setting names
    /// re-implements the policy and gets it subtly different, which is how a label
    /// editor came to ask for de-DE while the row held de.
    #[serde(rename = "locale_policy", default)]
    pub locale_policy: crate::models::MarketLocalePolicy,
    /// Every locale this market registers, in position order. Capped at 200. Empty
    /// is a real answer — read `default_locale` before assuming a language.
    #[serde(rename = "locales", default)]
    pub locales: Vec<crate::models::MarketLocale>,
    /// A distinct business context within a tenant — a country, a region, or a
    /// storefront segment such as B2C vs B2B — with its own base currency,
    /// locales, traded currencies and tax classes. A market is also the platform's
    /// `market` SCOPE dimension: every other commerce app slices its data by one,
    /// keyed on this row's `code`. A market is never just this row: it needs at
    /// least one locale, one currency and one tax class before it can serve, which
    /// is what /readiness measures and what /clone and /backfill build.
    #[serde(rename = "market", default)]
    pub market: crate::models::Market,
    /// Whether a stored price in this market is NET or GROSS — the market layer
    /// of an answer the prices app also holds. A price list's own tax_basis wins
    /// over this; `tax_basis: null` with `source: 'unset'` means this market
    /// declares nothing and the reader must fall through to the tenant's own
    /// default.
    #[serde(rename = "pricing", default)]
    pub pricing: crate::models::MarketPricing,
    /// Can this market actually trade? `ready` is false only when a BLOCKING check
    /// failed — no currency to quote in, no tax class to tax with. Warnings are
    /// degraded-but-serviceable.
    #[serde(rename = "readiness", default)]
    pub readiness: crate::models::MarketReadiness,
    /// Every tax class of this market with its rate, in position order. Capped at
    /// 200. This is the rate table other apps resolve a line against, by code.
    #[serde(rename = "tax_classes", default)]
    pub tax_classes: Vec<crate::models::MarketTaxClass>,
}
