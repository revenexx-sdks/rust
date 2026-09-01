use serde::{Deserialize, Serialize};

/// Counts, not rows: an import chunk of 5000 does not echo 5000 entries back.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceEntriesBulkResponse {
    /// Rows inserted — rungs this list did not have.
    #[serde(rename = "created", default)]
    pub created: i64,
    /// The mode actually applied — the request's, or the default `upsert`.
    #[serde(rename = "mode", default)]
    pub mode: String,
    /// Existing rungs rewritten in place (always 0 in append mode).
    #[serde(rename = "updated", default)]
    pub updated: i64,
}
