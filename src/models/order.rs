use serde::{Deserialize, Serialize};

/// An ORDER as it was placed: a snapshot. Buyer, addresses, payment and
/// shipping are frozen copies, the totals were computed here, and three
/// independent dimensions say where it stands — status (lifecycle),
/// payment_status (fed from outside) and fulfillment_status (derived from the
/// positions).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Order {
    /// When the fulfilling system took the order over. Written once. While it is
    /// null the order can still be modified here; afterwards modification goes
    /// through that system, unless the tenant sets
    /// allow_modification_after_acknowledge.
    #[serde(rename = "acknowledged_at", default)]
    pub acknowledged_at: String,
    /// The invoice address, FROZEN at place-time. Changing the customer's address
    /// afterwards does not change what this order was billed to.
    #[serde(rename = "billing_address", default)]
    pub billing_address: serde_json::Value,
    /// The ordering party as it was at place-time, FROZEN: a copy, not a
    /// reference, so the order still reads correctly after the customer record is
    /// renamed, merged or deleted. The caller decides what goes in; this app
    /// stores it and reads nothing out of it.
    #[serde(rename = "buyer", default)]
    pub buyer: serde_json::Value,
    /// When the order was cancelled, whether by a full cancel or by the last open
    /// quantity being cancelled position by position. Null otherwise.
    #[serde(rename = "cancelled_at", default)]
    pub cancelled_at: String,
    /// The cart this order was placed from, when a storefront handed one over. A
    /// reference across an app boundary (the carts app), not a foreign key —
    /// nothing here checks that it resolves. Null for an order an integration or
    /// an operator created.
    #[serde(rename = "cart_id", default)]
    pub cart_id: String,
    /// The sales channel the order arrived through — webshop, app, phone desk,
    /// EDI. Null when the caller named none.
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    /// When the order was closed — by a full shipment, by payment or by hand,
    /// depending on the tenant's auto_complete_on. Null until then.
    #[serde(rename = "completed_at", default)]
    pub completed_at: String,
    /// The PERSON who ordered — a contact in the customers app. Resolved from
    /// the acting principal whenever the caller carries one, and a body value that
    /// disagrees is refused rather than silently overridden. Null for a guest
    /// checkout.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// When the order row was written. For a placed order this is placed_at; for a
    /// requested one it is when the request was submitted.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// ISO 4217 code of EVERY amount on this order. Frozen at place-time from the
    /// market's default_currency unless the caller named one. Nothing on this
    /// order is ever converted, and the approval threshold is read in this
    /// currency — which is why the threshold is a per-market setting.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// The BUYER's own reference — their purchase-order number. Free text, not
    /// unique, never generated here: it exists so the paperwork can carry the
    /// number the buyer's accounts payable will look for. One of the few fields
    /// PUT /orders/{id} may still change.
    #[serde(rename = "customer_order_number", default)]
    pub customer_order_number: String,
    /// The FULFILLING system's reference for this order, typically the ERP order
    /// number. Written once by POST /orders/{id}/acknowledge and null until an
    /// integration acknowledged it.
    #[serde(rename = "external_ref", default)]
    pub external_ref: String,
    /// Whether the order has SHIPPED, and the one dimension nobody writes: it is
    /// DERIVED after every quantity change from the positions' own bookkeeping.
    /// 'fulfilled' means shipped >= ordered − cancelled across all positions,
    /// 'partial' means something went out. Sending it has no effect; ship, cancel
    /// or return something and it moves.
    #[serde(rename = "fulfillment_status", default)]
    pub fulfillment_status: String,
    /// What the buyer owes: subtotal + shipping_total + tax_total, COMPUTED by
    /// this app and NEVER taken from the caller — trusting a supplied total is
    /// how inconsistent orders happened. This is the number the approval threshold
    /// is compared against and the number the revenue rollup sums.
    #[serde(rename = "grand_total", default)]
    pub grand_total: f64,
    /// Why the order is held, in the words the shipping guard quotes back. Null
    /// when it is not held — releasing a hold clears it.
    #[serde(rename = "hold_reason", default)]
    pub hold_reason: String,
    /// Primary key of the order, and the id every other route takes. Not the order
    /// number.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The summed ORDERED quantity over all positions, rounded to a whole number
    /// — a headline figure for a list, computed once at place-time. It is
    /// deliberately not reduced when something is cancelled or returned; the
    /// positions carry that arithmetic.
    #[serde(rename = "item_count", default)]
    pub item_count: i64,
    /// Free-form data belonging to the INTEGRATION side — an ERP's own
    /// bookkeeping about this order. Stored and returned untouched; nothing here
    /// reads it.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// The order number a human quotes — drawn from the tenant's order range at
    /// place-time, unique per tenant and never reused. It is NOT the id: every
    /// route addresses an order by uuid, and GET /orders?number=… is how a
    /// number becomes one.
    #[serde(rename = "number", default)]
    pub number: String,
    /// A business stop, ORTHOGONAL to status: a held order keeps its lifecycle
    /// state and is refused at the guards. How far the hold reaches is the
    /// tenant's call (on_hold_blocks: shipping only, shipping and cancellation, or
    /// nothing at all).
    #[serde(rename = "on_hold", default)]
    pub on_hold: bool,
    /// The COMPANY the order is booked on — an organization in the customers
    /// app, and the B2B half of who ordered. This is what
    /// orders.reports.customer-rollup aggregates by and what makes an order
    /// visible to a buyer's colleagues. Null on a private or guest order, which
    /// the rollup counts separately because it cannot attribute it.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    /// The payment arrangement as it was chosen, FROZEN. This app reads exactly
    /// two keys and stores the rest untouched: 'status' seeds payment_status at
    /// place-time when it names one of the permitted values (anything else is
    /// ignored and the order starts 'open'), and 'payment_id' is merged in by POST
    /// /orders/{id}/payment-status. The method itself, its provider fields and any
    /// redirect state belong to the payments app.
    #[serde(rename = "payment", default)]
    pub payment: serde_json::Value,
    /// Whether the order is PAID, and the dimension this app does not decide: it
    /// is fed from outside through POST /orders/{id}/payment-status (the payments
    /// app or an ERP), and only seeded at place-time from payment.status.
    /// Orthogonal to the lifecycle — a completed order can still be open, and a
    /// paid one can still be pending.
    #[serde(rename = "payment_status", default)]
    pub payment_status: String,
    /// When the order was PLACED. Null while it is pending approval: an order
    /// awaiting sign-off exists but was never placed, and that is exactly the
    /// difference this field records.
    #[serde(rename = "placed_at", default)]
    pub placed_at: String,
    /// The shipping arrangement as it was chosen, FROZEN. Two keys are READ at
    /// place-time and feed the totals: 'price' becomes shipping_total (the
    /// shipping_total field is only the fallback when this is absent) and
    /// 'tax_rate' is what shipping is taxed at, because shipping is a
    /// Nebenleistung and is taxed too. Everything else — the carrier product,
    /// the delivery window, the pickup point — is stored untouched and belongs
    /// to the shipping app.
    #[serde(rename = "shipping", default)]
    pub shipping: serde_json::Value,
    /// The delivery address, FROZEN at place-time — what goes on the label of
    /// every shipment of this order. Null on an order that is never delivered (a
    /// service, a digital item, a collection).
    #[serde(rename = "shipping_address", default)]
    pub shipping_address: serde_json::Value,
    /// NET shipping cost, taken from shipping.price or, when the snapshot carries
    /// no price, from the request's shipping_total. In `currency`.
    #[serde(rename = "shipping_total", default)]
    pub shipping_total: f64,
    /// Where the order stands in its LIFECYCLE, and one of three independent
    /// status dimensions. 'pending' = created but not placed, an order waiting for
    /// approval; 'placed' = accepted, nothing shipped; 'in_fulfillment' = part of
    /// it has gone out, or all of it has and the tenant does not close on
    /// shipment; 'completed' and 'cancelled' end it. Moved by the action routes
    /// only — it is not writable through PUT /orders/{id}.
    #[serde(rename = "status", default)]
    pub status: String,
    /// NET total of the positions (the sum of their line_total), COMPUTED here at
    /// place-time. In `currency`, four decimal places. A caller cannot set it.
    #[serde(rename = "subtotal", default)]
    pub subtotal: f64,
    /// All tax on this order: the positions' tax_amount plus the tax on shipping
    /// (shipping_total × shipping.tax_rate). COMPUTED here — a caller cannot
    /// set it.
    #[serde(rename = "tax_total", default)]
    pub tax_total: f64,
    /// When any column of the order last changed — every status move, every
    /// re-derived fulfillment, every modification.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    /// Free-form data belonging to the ORDERING side — carried through from the
    /// storefront or the cart and handed back untouched. One of the few fields PUT
    /// /orders/{id} may still change.
    #[serde(rename = "user_data", default)]
    pub user_data: serde_json::Value,
}
