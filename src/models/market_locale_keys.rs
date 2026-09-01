use serde::{Deserialize, Serialize};

/// The read and write keys for one of the market's locales, already resolved
/// from the two settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketLocaleKeys {
    /// The market's locale this entry is about.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Its language part, which is also the key under language granularity.
    #[serde(rename = "language", default)]
    pub language: String,
    /// Keys to try in order until one holds text. Always starts at the exact code:
    /// a fallback fills a gap, it never outranks a stored value.
    #[serde(rename = "read", default)]
    pub read: Vec<String>,
    /// A key inside a labels bag: a full locale ('de-DE') under regional
    /// granularity, a bare language ('de') under language granularity.
    #[serde(rename = "write", default)]
    pub write: String,
}
