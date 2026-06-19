use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartMergeRequest {
    /// Cart whose lines move into the target (becomes status merged).
    #[serde(rename = "source_cart_id", default)]
    pub source_cart_id: String,
    /// Receiving cart (must be active).
    #[serde(rename = "target_cart_id", default)]
    pub target_cart_id: String,
}
