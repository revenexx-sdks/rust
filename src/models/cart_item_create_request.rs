use serde::{Deserialize, Serialize};

/// An item needs an identity: 'name' or 'sku'.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartItemCreateRequest {
    /// What was configured on this line, in the configurator's own vocabulary —
    /// this app stores it and reads nothing out of it. Its mere PRESENCE is
    /// behaviour: a line that carries a configuration never merges with another,
    /// because two differently configured units of the same article are not one
    /// line. Keys are the configurator's; the example is one shape, not the shape.
    #[serde(rename = "configuration", default)]
    pub configuration: serde_json::Value,
    /// ISO 4217 code. Defaults to the cart's currency.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// Free-form data the storefront hangs on the line. Stored and returned
    /// verbatim; no key in here is read by this app.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// What the line reads as on the cart page. Falls back to 'sku' when omitted,
    /// so a line always has something to show.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sort order within the cart, ascending. Default 0 when adding a line; in a
    /// bulk replace the payload order fills it in.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The catalogue product, when the line comes from one. Part of the merge
    /// identity: same product, same price, one line.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// How much of it — default 1. Fractional is legal (2.5 m of cable); zero
    /// and negative are not. On a plain product line that merges into an existing
    /// one, this is ADDED to what is already there, and max_quantity_per_line is
    /// checked on the result.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// The article number, exactly as the merchant knows it. Free text — this
    /// app does not resolve it against the catalogue — and part of the merge
    /// identity together with product_id and unit_price. The example only shows
    /// the shape of a real article number; nothing here enforces one.
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
    /// VAT percent for this line, as a number (19 means 19 %). Stored for the
    /// order to use — no total in this app includes tax.
    #[serde(rename = "tax_rate", default)]
    pub tax_rate: f64,
    /// Line type (default 'product'). Plain product lines merge by product+price;
    /// configurations always stand alone.
    #[serde(rename = "type", default)]
    pub xtype: String,
    /// The unit the quantity is counted in. Display and ERP hand-over only —
    /// this app converts nothing.
    #[serde(rename = "unit", default)]
    pub unit: String,
    /// Net price of one unit — line_total is always derived from it, never sent.
    /// Part of the merge identity: the same article at a different price opens a
    /// new line rather than averaging into the old one.
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
}
