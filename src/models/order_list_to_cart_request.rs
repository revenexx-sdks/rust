use serde::{Deserialize, Serialize};

/// Every field is optional: with an empty body the list goes into a NEW cart
/// for its owner, on the tenant defaults.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderListToCartRequest {
    /// Add to this existing cart. Omit to create one for the list owner and make
    /// it their current cart.
    #[serde(rename = "cart_id", default)]
    pub cart_id: String,
    /// ISO 4217 code for the cart and its lines. Omit to let the carts app decide.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// 'append' adds the positions (the carts app merges a line by product and
    /// price, so quantities accumulate); 'replace' makes the list the cart's
    /// entire contents. Defaults to the tenant's 'cart_merge_mode' setting.
    #[serde(rename = "mode", default)]
    pub mode: String,
}
