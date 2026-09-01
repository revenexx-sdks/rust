use serde::{Deserialize, Serialize};

/// The locale a storefront should render this market in. `source` names where
/// it came from: 'market' (a locale flagged is_default), 'market_first' (no
/// flag — first by position) or 'tenant_fallback' (the market registers
/// none; the tenant's fallback_locale setting answered).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketDefaultLocale {
    /// Locale code, language-COUNTRY — the language a storefront renders this
    /// market in, and the key a translation is stored under. Unique per market.
    /// The app's own seeded value is the tenant's `fallback_locale` setting, whose
    /// declared default is de-DE.
    #[serde(rename = "code", default)]
    pub code: String,
    /// ISO 3166-1 alpha-2 country code — the region half of `code`. It is a
    /// spelling of the language, not a shipping destination: a market may register
    /// de-AT without trading in Austria.
    #[serde(rename = "country", default)]
    pub country: String,
    /// ISO 639-1 language code — the language half of `code`, stored separately
    /// so a client can group markets by language without parsing.
    #[serde(rename = "language", default)]
    pub language: String,
    /// Which of the three rules answered. 'market' — a locale of this market
    /// carries is_default. 'market_first' — none does, so the first by position
    /// was taken. 'tenant_fallback' — the market registers no locale at all and
    /// the tenant's fallback_locale setting answered, which means this locale is
    /// NOT one of the market's own and nothing here was configured for it.
    #[serde(rename = "source", default)]
    pub source: String,
}
