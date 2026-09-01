use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartConversion {
    /// When the cart was abandoned — by hand, or by the cart-maintenance sweep.
    /// This is the only instant the abandonment funnel has, and nothing else in
    /// the platform writes it. carts.reopen clears it.
    #[serde(rename = "abandoned_at", default)]
    pub abandoned_at: String,
    /// The sales channel the cart was opened in (web shop, app, agent desk), as a
    /// channel of the channels app. Carried to the order for attribution; nothing
    /// in this app reads it.
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    /// The customer who owns this cart, as a contact of the customers app. Null on
    /// a guest cart: the database requires one of contact_id and session_key,
    /// never neither.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// When the cart was opened.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// ISO 4217 code the whole cart is priced in. A line added without a currency
    /// of its own inherits this one.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// The cart, as every other route addresses it. Stable for the cart's whole
    /// life: a merge closes a cart, it never renumbers one.
    #[serde(rename = "id", default)]
    pub id: String,
    /// THE current cart of this owner — the flag carts.activate writes, and
    /// reading it back is what `?is_current=true` is for. At most one cart per
    /// owner carries it: activating one clears it on every sibling, and
    /// abandoning, ordering or merging a cart clears it. A storefront resuming a
    /// session asks for it together with contact_id or session_key.
    #[serde(rename = "is_current", default)]
    pub is_current: bool,
    /// Total QUANTITY in the cart, not the number of lines: the sum of every
    /// line's quantity, rounded. Two lines of five pieces each answer 10, not 2.
    /// Recomputed by this app after every line write — a value a client sends is
    /// ignored.
    #[serde(rename = "item_count", default)]
    pub item_count: i64,
    /// The market this cart is scoped to, stamped by the platform. It decides
    /// which market's settings apply — including the retention windows the sweep
    /// deletes on. Null on a cart that belongs to no market, which runs on the
    /// tenant baseline. Cart lines and io profiles carry no market of their own; a
    /// line's market is its cart's.
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    /// The cart this one was merged into, written together with status 'merged'.
    /// The lines are in the target now and this is the trail back — the answer
    /// to 'where did my cart go'. Null on every cart that was never merged.
    #[serde(rename = "merged_into_cart_id", default)]
    pub merged_into_cart_id: String,
    /// Free-form data the storefront hangs on the cart. Stored and returned
    /// verbatim; no key in here is read by this app, and none is indexed.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// What the buyer calls this cart. B2B customers keep several named carts side
    /// by side — 'Weekly order', 'Site B', 'Q3 budget' — which is what
    /// multi_cart_enabled turns on; a storefront with one cart per buyer leaves it
    /// at the default 'Cart'.
    #[serde(rename = "name", default)]
    pub name: String,
    /// The order this cart became, in whatever numbering order management uses.
    /// Free text: this app stores what it is handed and never resolves it.
    /// Filtering on it is how a support agent gets from an order number back to
    /// the cart behind it.
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
    /// When the cart was handed to order management. Written once, with the
    /// status, and never cleared.
    #[serde(rename = "ordered_at", default)]
    pub ordered_at: String,
    /// How price_snapshot_mode settled the two prices every line carries.
    #[serde(rename = "pricing", default)]
    pub pricing: crate::models::CartConversionPricing,
    /// What this app ASKED inventories for, and what it answered. This app holds
    /// no stock: inventories picks the location, applies the backorder policy and
    /// owns the hold's expiry.
    #[serde(rename = "reservation", default)]
    pub reservation: crate::models::CartConversionReservation,
    /// How a cart is identified BEFORE anyone logs in — the opaque key the
    /// storefront already keeps in its own session or cookie and sends back on
    /// every anonymous call. This app neither issues nor parses it; any non-empty
    /// string is a valid key, so its format is the storefront's own. On login
    /// carts.claim hands every active cart of one session_key to a contact, and
    /// this becomes null.
    #[serde(rename = "session_key", default)]
    pub session_key: String,
    /// Where the cart stands in its lifecycle. 'active' is the only status that
    /// accepts a write of any kind. 'abandoned' is set by hand or by the
    /// cart-maintenance sweep and is the one reversible ending (carts.reopen).
    /// 'ordered' and 'merged' are final — the cart is a record now, not a
    /// workspace.
    #[serde(rename = "status", default)]
    pub status: String,
    /// Sum of every line's line_total, in the cart's currency, net — before
    /// shipping, before tax. Recomputed after every line write, and written once
    /// more by carts.order when price_snapshot_mode settles which of a line's two
    /// prices is charged.
    #[serde(rename = "subtotal", default)]
    pub subtotal: f64,
    /// The tenant this row belongs to, echoed by the data plane. Always the tenant
    /// the request was made for — it is not a way to reach another one.
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    /// The last time anything about this cart or its lines changed — every write
    /// path in this app stamps it. It is also what the maintenance sweep measures
    /// idleness with, which is why the abandonment sweep is the one write that
    /// deliberately does not touch it: noticing that a cart is idle must not reset
    /// the clock that decides how long it is kept.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
