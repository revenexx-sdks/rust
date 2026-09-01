use serde::{Deserialize, Serialize};

/// Every field is optional — the buyer, the organization and the positions
/// all come from the list.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderListToOrderRequest {
    /// ISO 4217 code. Omit to let the orders app apply the market default.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// The BUYER's own order or purchase-order number, forwarded to the orders app
    /// verbatim. Free text and never generated here: it exists so the paperwork
    /// can carry the number the buyer's accounts payable will look for.
    #[serde(rename = "customer_order_number", default)]
    pub customer_order_number: String,
}
