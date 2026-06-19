use serde::{Deserialize, Serialize};

/// The order aggregate: every column of the order plus its items, shipments
/// (with positions), returns and cancellations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderDetail {
    #[serde(rename = "acknowledged_at", default)]
    pub acknowledged_at: String,
    #[serde(rename = "billing_address", default)]
    pub billing_address: serde_json::Value,
    #[serde(rename = "buyer", default)]
    pub buyer: serde_json::Value,
    #[serde(rename = "cancellations", default)]
    pub cancellations: Vec<crate::models::OrderCancellation>,
    #[serde(rename = "cancelled_at", default)]
    pub cancelled_at: String,
    #[serde(rename = "cart_id", default)]
    pub cart_id: String,
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    #[serde(rename = "completed_at", default)]
    pub completed_at: String,
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "currency", default)]
    pub currency: String,
    #[serde(rename = "customer_order_number", default)]
    pub customer_order_number: String,
    #[serde(rename = "external_ref", default)]
    pub external_ref: String,
    #[serde(rename = "fulfillment_status", default)]
    pub fulfillment_status: String,
    #[serde(rename = "grand_total", default)]
    pub grand_total: f64,
    #[serde(rename = "hold_reason", default)]
    pub hold_reason: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "item_count", default)]
    pub item_count: i64,
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::OrderItem>,
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "number", default)]
    pub number: String,
    #[serde(rename = "on_hold", default)]
    pub on_hold: bool,
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    #[serde(rename = "payment", default)]
    pub payment: serde_json::Value,
    #[serde(rename = "payment_status", default)]
    pub payment_status: String,
    #[serde(rename = "placed_at", default)]
    pub placed_at: String,
    #[serde(rename = "returns", default)]
    pub returns: Vec<crate::models::OrderReturn>,
    #[serde(rename = "shipments", default)]
    pub shipments: Vec<crate::models::OrderShipment>,
    #[serde(rename = "shipping", default)]
    pub shipping: serde_json::Value,
    #[serde(rename = "shipping_address", default)]
    pub shipping_address: serde_json::Value,
    #[serde(rename = "shipping_total", default)]
    pub shipping_total: f64,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "subtotal", default)]
    pub subtotal: f64,
    #[serde(rename = "tax_total", default)]
    pub tax_total: f64,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    #[serde(rename = "user_data", default)]
    pub user_data: serde_json::Value,
}
