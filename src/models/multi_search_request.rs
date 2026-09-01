use serde::{Deserialize, Serialize};

/// Envelope for a federated search. Top-level search parameters outside
/// `searches` are forwarded to Typesense unchanged and act as defaults for
/// every entry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MultiSearchRequest {
    /// The searches to run, in order. Must not be empty.
    #[serde(rename = "searches", default)]
    pub searches: Vec<crate::models::MultiSearchEntry>,
}
