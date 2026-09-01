use serde::{Deserialize, Serialize};

/// Identify by 'product_id' or 'sku' — an item without identity resolves to
/// on_request with a per-item error rather than failing the call.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceResolveItem {
    /// Product to price.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Requested quantity, counted in the entry’s `unit`. It picks the tier (the
    /// highest `quantity_min` at or below it) and multiplies into `line_total`.
    /// Default 1; a non-positive value falls back to 1.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// SKU to price (alternative to product_id). Matched exactly against the
    /// entries’ own `sku`.
    #[serde(rename = "sku", default)]
    pub sku: String,
}
