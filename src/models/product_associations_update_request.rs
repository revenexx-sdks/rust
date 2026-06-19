use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductAssociationsUpdateRequest {
    #[serde(rename = "association_type_id", default)]
    pub association_type_id: String,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    #[serde(rename = "target_product_id", default)]
    pub target_product_id: String,
}
