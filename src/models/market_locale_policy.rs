use serde::{Deserialize, Serialize};

/// How this tenant keys its translations, resolved rather than named: the key
/// a client WRITES and the order it READS, per locale. Emitting the resolved
/// answer is the point — a client handed only the setting names
/// re-implements the policy and gets it subtly different, which is how a label
/// editor came to ask for de-DE while the row held de.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketLocalePolicy {
    /// settings#locale_fallback — what a read tries after the exact key holds
    /// nothing.
    #[serde(rename = "fallback", default)]
    pub fallback: String,
    /// settings#locale_granularity — whether a value is keyed by the full locale
    /// ('regional') or by its language alone.
    #[serde(rename = "granularity", default)]
    pub granularity: String,
    /// One entry per locale this market registers, in position order — the keys
    /// to use for that locale. A market with no locale of its own has an empty
    /// array here, not the fallback: the fallback answers `default_locale`, and
    /// there is nothing to key against.
    #[serde(rename = "locales", default)]
    pub locales: Vec<crate::models::MarketLocaleKeys>,
}
