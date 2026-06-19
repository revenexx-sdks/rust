use serde::{Deserialize, Serialize};

/// Identify by 'product_id' or 'sku' — an item without identity resolves to
/// on_request with a per-item error.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceResolveItem {
    /// Product to price.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Requested quantity for tier selection and line_total (default 1;
    /// non-positive values fall back to 1).
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// SKU to price (alternative to product_id).
    #[serde(rename = "sku", default)]
    pub sku: String,
}
