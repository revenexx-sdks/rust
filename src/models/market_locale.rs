use serde::{Deserialize, Serialize};

/// One language a market is rendered in, and one key its translations are
/// stored under. A market may register several; one of them is the default a
/// storefront falls back to.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketLocale {
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
    /// When the locale was registered on this market. Set by the database; never
    /// writable.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Primary key of this locale registration. The locale is named by `code`
    /// everywhere else.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The locale a storefront renders this market in when the request asks for
    /// none. At most one per market; where none carries the flag the first by
    /// position is used, and `default_locale.source` on the context says which of
    /// the two happened.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// ISO 639-1 language code — the language half of `code`, stored separately
    /// so a client can group markets by language without parsing.
    #[serde(rename = "language", default)]
    pub language: String,
    /// The market this locale belongs to. Filled from the route path on write and
    /// never read out of the body; ON DELETE CASCADE, so deleting the market
    /// deletes this row.
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    /// Sort position among this market's locales, ascending, default 0 — and the
    /// tie-break that picks a default when no locale is flagged.
    #[serde(rename = "position", default)]
    pub position: i64,
}
