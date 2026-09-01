use serde::{Deserialize, Serialize};

/// Narrow modification — these six columns and no others. Anything else in
/// the body is ignored, and a body with none of them at all is a 400 naming
/// the allowed set. A whole key REPLACES the value it names; there is no merge
/// into an existing snapshot. Nothing here moves the order: status, payment
/// and fulfillment travel through the action routes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderUpdateRequest {
    /// The invoice address, FROZEN at place-time. Changing the customer's address
    /// afterwards does not change what this order was billed to. Replaced
    /// wholesale — send the whole address, not a patch of it.
    #[serde(rename = "billing_address", default)]
    pub billing_address: serde_json::Value,
    /// The ordering party as it was at place-time, FROZEN: a copy, not a
    /// reference, so the order still reads correctly after the customer record is
    /// renamed, merged or deleted. The caller decides what goes in; this app
    /// stores it and reads nothing out of it. Replaced wholesale — send the
    /// whole snapshot, not a patch of it.
    #[serde(rename = "buyer", default)]
    pub buyer: serde_json::Value,
    /// The BUYER's own reference — their purchase-order number. Free text, not
    /// unique, never generated here: it exists so the paperwork can carry the
    /// number the buyer's accounts payable will look for. One of the few fields
    /// PUT /orders/{id} may still change.
    #[serde(rename = "customer_order_number", default)]
    pub customer_order_number: String,
    /// Free-form data belonging to the INTEGRATION side — an ERP's own
    /// bookkeeping about this order. Stored and returned untouched; nothing here
    /// reads it. Replaced wholesale.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// The delivery address, FROZEN at place-time — what goes on the label of
    /// every shipment of this order. Null on an order that is never delivered (a
    /// service, a digital item, a collection). Replaced wholesale. This is the one
    /// correction that actually matters after placement: the label of every
    /// shipment still to go out is printed from it.
    #[serde(rename = "shipping_address", default)]
    pub shipping_address: serde_json::Value,
    /// Free-form data belonging to the ORDERING side — carried through from the
    /// storefront or the cart and handed back untouched. One of the few fields PUT
    /// /orders/{id} may still change. Replaced wholesale.
    #[serde(rename = "user_data", default)]
    pub user_data: serde_json::Value,
}
