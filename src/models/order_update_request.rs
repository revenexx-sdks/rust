use serde::{Deserialize, Serialize};

/// Narrow modification — only these columns are touchable, and only until
/// the order is acknowledged. Status moves through the action routes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderUpdateRequest {
    #[serde(rename = "billing_address", default)]
    pub billing_address: serde_json::Value,
    #[serde(rename = "buyer", default)]
    pub buyer: serde_json::Value,
    #[serde(rename = "customer_order_number", default)]
    pub customer_order_number: String,
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "shipping_address", default)]
    pub shipping_address: serde_json::Value,
    /// Free-form user data.
    #[serde(rename = "user_data", default)]
    pub user_data: serde_json::Value,
}
