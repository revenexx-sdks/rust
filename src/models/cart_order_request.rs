use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartOrderRequest {
    /// External order reference from order management.
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
}
