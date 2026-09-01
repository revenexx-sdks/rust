use serde::{Deserialize, Serialize};

/// The exact-column filters this call applied, echoed back. Every value is the
/// raw query string, never the column's own type: `?is_default=true` comes
/// back as `"true"`. A `?column=value` naming a column this entity does not
/// have is DROPPED rather than refused — the call answers 200 with the
/// unfiltered list, and the key missing from here is the only way to find out.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketLocaleFilter {
    /// The `code` filter as it arrived, verbatim. Present only when the call sent
    /// it.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The `country` filter as it arrived, verbatim. Present only when the call
    /// sent it.
    #[serde(rename = "country", default)]
    pub country: String,
    /// The `created_at` filter as it arrived, verbatim. Present only when the call
    /// sent it. Any form the database accepts as a timestamp, including a bare
    /// date.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The `id` filter as it arrived, verbatim. Present only when the call sent
    /// it.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The `is_default` filter as it arrived, verbatim. Present only when the call
    /// sent it.
    #[serde(rename = "is_default", default)]
    pub is_default: String,
    /// The `language` filter as it arrived, verbatim. Present only when the call
    /// sent it.
    #[serde(rename = "language", default)]
    pub language: String,
    /// The owning market, taken from the route path. ALWAYS present, and always
    /// the path's market — a `?market_id=` in the query is overwritten by it
    /// rather than honoured, so this is never the value a caller sent.
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    /// The `position` filter as it arrived, verbatim. Present only when the call
    /// sent it.
    #[serde(rename = "position", default)]
    pub position: String,
}
