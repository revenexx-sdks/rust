use serde::{Deserialize, Serialize};

/// A position of the placed order — needs an identity: 'name' or 'sku'.
/// Items are SNAPSHOTS: carry the product copy, prices are frozen at
/// place-time.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderItemCreateRequest {
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
    /// Free-form data belonging to the integration side, per position. Stored and
    /// returned untouched.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// The article name as it stood at place-time, frozen. Falls back to the sku
    /// when the caller sent none — a position always reads as something. Falls
    /// back to 'sku' when omitted; one of the two is required.
    #[serde(rename = "name", default)]
    pub name: String,
    /// The line number a human reads, and what the order is sorted by. Numbered in
    /// steps of the range's position_step (10, 20, 30) unless the caller set it
    /// explicitly — the gap is what lets a line be inserted later without
    /// renumbering. Omitted = numbered in steps of the order range's
    /// position_step.
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
    /// stores it and reads nothing out of it. 'snapshot' is accepted as an alias
    /// for this key.
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
    /// truthful record of what was asked for. Defaults to 1.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// The article number as it stood at place-time, frozen with the rest of the
    /// line. The value an ERP and a warehouse both join on, and the one field a
    /// picker reads. Null only on a line that never had one.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// The product as it was at place-time, FROZEN: the copy that makes the order
    /// still correct after the catalog changes its price, its name or its
    /// attributes. The caller decides how much of the product to freeze; this app
    /// stores it and reads nothing out of it. Alias for 'product' — send one or
    /// the other, not both.
    #[serde(rename = "snapshot", default)]
    pub snapshot: serde_json::Value,
    /// Tax on this line in `currency`. Derived from line_total × tax_rate/100
    /// when the caller sent none, which is the normal case — but a caller may
    /// send it, for a market whose rounding rules differ from ours. Send it only
    /// where your market rounds differently from line_total × tax_rate/100.
    #[serde(rename = "tax_amount", default)]
    pub tax_amount: f64,
    /// Tax percentage for this line, as a number (19 means 19 %). Frozen at
    /// place-time with everything else. Defaults to 0.
    #[serde(rename = "tax_rate", default)]
    pub tax_rate: f64,
    /// What kind of line this is: 'product' is a catalog article, 'configuration'
    /// a configured one carrying its configuration, 'custom' a line typed by hand
    /// that no catalog knows. Defaults to 'product'.
    #[serde(rename = "type", default)]
    pub xtype: String,
    /// The unit the quantity is counted in — piece, metre, kilogram, package.
    /// Free text as the catalog carries it; this app does no conversion.
    #[serde(rename = "unit", default)]
    pub unit: String,
    /// NET price per unit, FROZEN at place-time. A later price change in the
    /// catalog does not reach this order. Defaults to 0. line_total is always
    /// derived from it and never taken from the body.
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
    /// Free-form data belonging to the ordering side, per position — carried
    /// through from the cart line and handed back untouched.
    #[serde(rename = "user_data", default)]
    pub user_data: serde_json::Value,
}
