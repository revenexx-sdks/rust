use serde::{Deserialize, Serialize};

/// Name the products either way, or both ways. Send at least one non-empty
/// list; the two are unioned and a product named twice comes back once.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductsBatchRequest {
    /// Product ids, when the caller already holds them.
    #[serde(rename = "ids", default)]
    pub ids: Vec<String>,
    /// Product SKUs — the identifier a foreign system carries, which is why this
    /// route exists at all.
    #[serde(rename = "skus", default)]
    pub skus: Vec<String>,
}
