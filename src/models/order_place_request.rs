use serde::{Deserialize, Serialize};

/// The snapshot payload: items plus frozen buyer/addresses/payment/shipping.
/// The order number is drawn from the order range, totals are computed from
/// the items.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderPlaceRequest {
    /// Frozen billing address.
    #[serde(rename = "billing_address", default)]
    pub billing_address: serde_json::Value,
    /// Frozen buyer snapshot (name, email, …).
    #[serde(rename = "buyer", default)]
    pub buyer: serde_json::Value,
    /// Source cart (the carts.order hand-over).
    #[serde(rename = "cart_id", default)]
    pub cart_id: String,
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    /// Ordering customer contact.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// ISO 4217 code (default EUR).
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// The buyer's own order/PO number.
    #[serde(rename = "customer_order_number", default)]
    pub customer_order_number: String,
    /// Override — computed as subtotal + shipping + tax when omitted.
    #[serde(rename = "grand_total", default)]
    pub grand_total: f64,
    /// The order positions (at most 500).
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::OrderItemCreateRequest>,
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// B2B organization.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    /// Frozen payment snapshot — a known 'payment.status' seeds payment_status
    /// (otherwise 'open').
    #[serde(rename = "payment", default)]
    pub payment: serde_json::Value,
    /// Frozen shipping snapshot — 'shipping.price' seeds shipping_total.
    #[serde(rename = "shipping", default)]
    pub shipping: serde_json::Value,
    /// Frozen shipping address.
    #[serde(rename = "shipping_address", default)]
    pub shipping_address: serde_json::Value,
    /// Shipping total (fallback when 'shipping.price' is absent).
    #[serde(rename = "shipping_total", default)]
    pub shipping_total: f64,
    /// Free-form user data.
    #[serde(rename = "user_data", default)]
    pub user_data: serde_json::Value,
}
