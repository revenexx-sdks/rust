use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartItem {
    /// The cart this line belongs to. A line never moves between carts — a merge
    /// copies it into the target and closes the source cart.
    #[serde(rename = "cart_id", default)]
    pub cart_id: String,
    /// What was configured on this line, in the configurator's own vocabulary —
    /// this app stores it and reads nothing out of it. Its mere PRESENCE is
    /// behaviour: a line that carries a configuration never merges with another,
    /// because two differently configured units of the same article are not one
    /// line. Keys are the configurator's; the example is one shape, not the shape.
    #[serde(rename = "configuration", default)]
    pub configuration: serde_json::Value,
    /// When the line was added. A merge into an existing line keeps the original
    /// — the quantity moved, the line did not.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// ISO 4217 code this line is priced in. Defaults to the cart's currency when
    /// a line is added without one.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// The line, as carts.items.get/update/delete address it.
    #[serde(rename = "id", default)]
    pub id: String,
    /// quantity × unit_price, net, always derived. A line_total in a payload is
    /// ignored: the cart may not disagree with its own arithmetic.
    #[serde(rename = "line_total", default)]
    pub line_total: f64,
    /// Free-form data the storefront hangs on the line. Stored and returned
    /// verbatim; no key in here is read by this app.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// What the line reads as on the cart page. Falls back to the SKU when a
    /// caller sends none, so a line always has something to show.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sort order within the cart, ascending. Lines come back in this order unless
    /// `order` says otherwise, and a bulk replace numbers them by their place in
    /// the payload.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The catalogue product this line came from, when it came from one. Null on a
    /// custom line, and null on a product line the storefront identified by SKU
    /// alone.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// How much of it. Fractional on purpose — 2.5 metres of cable is a line,
    /// not a rounding error — and always greater than zero: removing a line is a
    /// DELETE, not a quantity of 0.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// The article number the merchant sorts by in the ERP — the value every
    /// integration joins on. Free text here: this app does not resolve it against
    /// the catalogue, so it is exactly what the storefront wrote into the line.
    /// Together with product_id and unit_price it decides whether adding the same
    /// article again lands on this line or opens a new one.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// The product as the buyer was shown it when this line was added — the
    /// cart's own copy, so it stays honest when the catalogue moves underneath it.
    /// Free-form apart from the price: conversion reads `unit_price` (or `price`
    /// as a fallback) and nothing else. A snapshot without a readable price leaves
    /// the line alone in both price modes, which is deliberate — a missing
    /// snapshot must never be read as "free".
    #[serde(rename = "snapshot", default)]
    pub snapshot: crate::models::CartItemSnapshot,
    /// VAT percent for this line, as a number (19 means 19 %). Stored with the
    /// line for the order to use — no total in this app includes tax.
    #[serde(rename = "tax_rate", default)]
    pub tax_rate: f64,
    /// The tenant this row belongs to, echoed by the data plane.
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    /// What kind of line this is. 'product' is a catalogue line and the only type
    /// that ever merges with another. 'configuration' is a configured product —
    /// it carries its configuration and always stands alone, because two
    /// differently configured units of the same article are not the same line.
    /// 'custom' is a free line nobody has to find in a catalogue: a service, a
    /// surcharge, a hand-typed position.
    #[serde(rename = "type", default)]
    pub xtype: String,
    /// The unit the quantity is counted in ('pcs', 'm', 'kg', 'h'). Display and
    /// ERP hand-over only; this app converts nothing.
    #[serde(rename = "unit", default)]
    pub unit: String,
    /// Net price of ONE unit, in the line's currency. This is the working price
    /// — a resync, a PUT on the line or a repricing job may have moved it since
    /// the buyer saw it. The price the buyer WAS shown lives in snapshot, and
    /// carts.order decides which of the two the order is booked on.
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
    /// When the line last changed — including a quantity another add merged into
    /// it.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
