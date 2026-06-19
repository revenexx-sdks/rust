use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartItemsReplaceRequest {
    /// The complete new item set (set semantics).
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::CartItemCreateRequest>,
}
