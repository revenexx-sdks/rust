use serde::{Deserialize, Serialize};

/// Typesense search parameters. Only the commonly used ones are enumerated —
/// the proxy forwards the whole object, so any parameter Typesense accepts may
/// be sent.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchParameters {
    /// Comma-separated document fields to omit.
    #[serde(rename = "exclude_fields", default)]
    pub exclude_fields: String,
    /// Comma-separated fields to facet on.
    #[serde(rename = "facet_by", default)]
    pub facet_by: String,
    /// Filter expression, e.g. `in_stock:=true && price:<100`. ANDed with the
    /// tenant filter the proxy injects.
    #[serde(rename = "filter_by", default)]
    pub filter_by: String,
    /// Comma-separated fields to group results by.
    #[serde(rename = "group_by", default)]
    pub group_by: String,
    /// Comma-separated fields to highlight in full.
    #[serde(rename = "highlight_full_fields", default)]
    pub highlight_full_fields: String,
    /// Comma-separated document fields to return.
    #[serde(rename = "include_fields", default)]
    pub include_fields: String,
    /// Facet values to return per field.
    #[serde(rename = "max_facet_values", default)]
    pub max_facet_values: i64,
    /// Typos tolerated per query token.
    #[serde(rename = "num_typos", default)]
    pub num_typos: i64,
    /// 1-based page number.
    #[serde(rename = "page", default)]
    pub page: i64,
    /// Hits per page.
    #[serde(rename = "per_page", default)]
    pub per_page: i64,
    /// Whether the last token is a prefix; per-field when comma-separated.
    #[serde(rename = "prefix", default)]
    pub prefix: String,
    /// Query text. Use `*` to match everything.
    #[serde(rename = "q", default)]
    pub q: String,
    /// Comma-separated fields to search, in weight order.
    #[serde(rename = "query_by", default)]
    pub query_by: String,
    /// Sort expression, e.g. `price:desc`.
    #[serde(rename = "sort_by", default)]
    pub sort_by: String,
}
