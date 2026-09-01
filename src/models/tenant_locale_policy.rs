use serde::{Deserialize, Serialize};

/// How this tenant keys its translations, resolved rather than named: the key
/// a client WRITES and the order it READS, per locale. Emitting the resolved
/// answer is the point — a client handed only the setting names
/// re-implements the policy and gets it subtly different, which is how a label
/// editor came to ask for de-DE while the row held de.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TenantLocalePolicy {
    /// settings#locale_fallback — what a read tries after the exact key holds
    /// nothing.
    #[serde(rename = "fallback", default)]
    pub fallback: String,
    /// settings#locale_granularity — whether a value is keyed by the full locale
    /// ('regional') or by its language alone.
    #[serde(rename = "granularity", default)]
    pub granularity: String,
    /// The UNION of every market's locales, each one appearing once — the full
    /// set of inputs a tenant-baseline editor has to offer. Empty when no market
    /// registers a locale at all.
    #[serde(rename = "locales", default)]
    pub locales: Vec<crate::models::TenantLocaleKeys>,
}
