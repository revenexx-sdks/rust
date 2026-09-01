use serde::{Deserialize, Serialize};

/// One order position with the quantity that may still be shipped, and the
/// three numbers that quantity is made of. Every position of the order is
/// here, including the ones with nothing left open — a dialog needs to show
/// a fully shipped line as fully shipped, not omit it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderShippablePosition {
    /// The article name as it stood at place-time, frozen. Falls back to the sku
    /// when the caller sent none — a position always reads as something.
    #[serde(rename = "name", default)]
    pub name: String,
    /// The position, by the id a positions[] payload names it with. This is what
    /// POST /orders/{id}/ship expects — copy it, do not construct it.
    #[serde(rename = "order_item_id", default)]
    pub order_item_id: String,
    /// The line number a human reads, and what the order is sorted by. Numbered in
    /// steps of the range's position_step (10, 20, 30) unless the caller set it
    /// explicitly — the gap is what lets a line be inserted later without
    /// renumbering.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The catalog product this line was taken from (the products app). Null on a
    /// custom line, and it stays a reference — the position keeps working after
    /// the product is retired.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// How much was ORDERED on this position. Unchanged by anything that happens
    /// afterwards.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// How much was cancelled and will never go out.
    #[serde(rename = "quantity_cancelled", default)]
    pub quantity_cancelled: f64,
    /// quantity − shipped − cancelled: the budget POST /orders/{id}/ship
    /// guards this position against, and the largest quantity it will accept. Zero
    /// means the line is done.
    #[serde(rename = "quantity_open", default)]
    pub quantity_open: f64,
    /// How much has already gone out.
    #[serde(rename = "quantity_shipped", default)]
    pub quantity_shipped: f64,
    /// The article number as it stood at place-time, frozen with the rest of the
    /// line. The value an ERP and a warehouse both join on, and the one field a
    /// picker reads. Null only on a line that never had one.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// The unit the quantity is counted in — piece, metre, kilogram, package.
    /// Free text as the catalog carries it; this app does no conversion.
    #[serde(rename = "unit", default)]
    pub unit: String,
}
