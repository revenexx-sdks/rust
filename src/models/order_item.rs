use serde::{Deserialize, Serialize};

/// One POSITION of an order, frozen at place-time: the article as it was, the
/// price as it was, and three running quantities (shipped, cancelled,
/// returned) that everything after placement books against. `quantity` itself
/// never changes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderItem {
    /// The chosen options of a configured line — what the configurator produced,
    /// in whatever shape it produces. Only meaningful for type 'configuration';
    /// null everywhere else.
    #[serde(rename = "configuration", default)]
    pub configuration: serde_json::Value,
    /// The buyer's own cost centre for this line — a B2B field: the same order
    /// is split across several of them and the buyer's finance department needs
    /// the split per line, not per order.
    #[serde(rename = "cost_center", default)]
    pub cost_center: String,
    /// When the position was written — the moment the order was placed.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Primary key of the position. This is the id every positions[] payload
    /// names: /ship, /items/cancel and /return all take order_item_id.
    #[serde(rename = "id", default)]
    pub id: String,
    /// quantity × unit_price, NET, always COMPUTED here — a caller cannot set
    /// it. The order's subtotal is the sum of these.
    #[serde(rename = "line_total", default)]
    pub line_total: f64,
    /// Free-form data belonging to the integration side, per position. Stored and
    /// returned untouched.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// The article name as it stood at place-time, frozen. Falls back to the sku
    /// when the caller sent none — a position always reads as something.
    #[serde(rename = "name", default)]
    pub name: String,
    /// The order this position belongs to. Deleting the order deletes its
    /// positions.
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    /// The line number a human reads, and what the order is sorted by. Numbered in
    /// steps of the range's position_step (10, 20, 30) unless the caller set it
    /// explicitly — the gap is what lets a line be inserted later without
    /// renumbering.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// A free note the buyer attached to this line — an engraving, a delivery
    /// instruction, the drawing number the line refers to. Printed on the
    /// paperwork, read by nothing.
    #[serde(rename = "position_text", default)]
    pub position_text: String,
    /// The product as it was at place-time, FROZEN: the copy that makes the order
    /// still correct after the catalog changes its price, its name or its
    /// attributes. The caller decides how much of the product to freeze; this app
    /// stores it and reads nothing out of it.
    #[serde(rename = "product", default)]
    pub product: serde_json::Value,
    /// The catalog product this line was taken from (the products app). Null on a
    /// custom line, and it stays a reference — the position keeps working after
    /// the product is retired.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// How much was ORDERED, in `unit`. Three decimal places, so 2.5 m of cable is
    /// a real order line. Never changed afterwards — cancelling or returning
    /// writes the quantity_* columns instead, which is what keeps the order a
    /// truthful record of what was asked for.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// How much of this position was cancelled and will never ship. Written by
    /// /cancel (all of it) and /items/cancel (a named quantity). Cancelling
    /// reduces the effective quantity, so an order whose every position is fully
    /// cancelled becomes cancelled itself.
    #[serde(rename = "quantity_cancelled", default)]
    pub quantity_cancelled: f64,
    /// How much of this position came BACK, booked when a return is completed —
    /// not when it is registered or received. This is the goods accounting: it
    /// never reduces quantity_shipped, so a position can be shipped 3 and returned
    /// 3.
    #[serde(rename = "quantity_returned", default)]
    pub quantity_returned: f64,
    /// How much of this position has GONE OUT, summed over the shipments. Written
    /// only by POST /orders/{id}/ship; it is what fulfillment_status is derived
    /// from, and what a return is guarded against.
    #[serde(rename = "quantity_shipped", default)]
    pub quantity_shipped: f64,
    /// The article number as it stood at place-time, frozen with the rest of the
    /// line. The value an ERP and a warehouse both join on, and the one field a
    /// picker reads. Null only on a line that never had one.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// Tax on this line in `currency`. Derived from line_total × tax_rate/100
    /// when the caller sent none, which is the normal case — but a caller may
    /// send it, for a market whose rounding rules differ from ours.
    #[serde(rename = "tax_amount", default)]
    pub tax_amount: f64,
    /// Tax percentage for this line, as a number (19 means 19 %). Frozen at
    /// place-time with everything else.
    #[serde(rename = "tax_rate", default)]
    pub tax_rate: f64,
    /// What kind of line this is: 'product' is a catalog article, 'configuration'
    /// a configured one carrying its configuration, 'custom' a line typed by hand
    /// that no catalog knows.
    #[serde(rename = "type", default)]
    pub xtype: String,
    /// The unit the quantity is counted in — piece, metre, kilogram, package.
    /// Free text as the catalog carries it; this app does no conversion.
    #[serde(rename = "unit", default)]
    pub unit: String,
    /// NET price per unit, FROZEN at place-time. A later price change in the
    /// catalog does not reach this order.
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
    /// When the position last changed, which in practice means the last time a
    /// quantity was booked onto it.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    /// Free-form data belonging to the ordering side, per position — carried
    /// through from the cart line and handed back untouched.
    #[serde(rename = "user_data", default)]
    pub user_data: serde_json::Value,
}
