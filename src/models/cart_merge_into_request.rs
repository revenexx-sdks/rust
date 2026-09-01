use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartMergeIntoRequest {
    /// Receiving cart (must be active). The cart in the path is the source and
    /// becomes status merged.
    #[serde(rename = "target_cart_id", default)]
    pub target_cart_id: String,
}
