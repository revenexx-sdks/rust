use serde::{Deserialize, Serialize};

/// One locale somewhere in this tenant, its read and write keys, and the
/// markets that asked for it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TenantLocaleKeys {
    /// The locale this entry is about, as some market registered it.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Its language part, which is also the key under language granularity.
    #[serde(rename = "language", default)]
    pub language: String,
    /// Codes of the markets that registered this locale, sorted — who a baseline
    /// translation written here is actually for. An editor that lists six inputs
    /// without saying who needs them invites translations nobody will ever read.
    #[serde(rename = "markets", default)]
    pub markets: Vec<String>,
    /// Keys to try in order until one holds text — the same resolved order the
    /// per-market answer gives, so a baseline value and a market value can never
    /// be keyed differently.
    #[serde(rename = "read", default)]
    pub read: Vec<String>,
    /// A key inside a labels bag: a full locale ('de-DE') under regional
    /// granularity, a bare language ('de') under language granularity.
    #[serde(rename = "write", default)]
    pub write: String,
}
