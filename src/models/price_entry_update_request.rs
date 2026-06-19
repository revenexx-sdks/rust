use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceEntryUpdateRequest {
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Default 'standard'; 'on_request' is the explicit no-price marker — it
    /// stops resolution and answers "price on request".
    #[serde(rename = "price_type", default)]
    pub price_type: String,
    /// Priced product.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Tier threshold (Staffelpreis): this price applies from this quantity
    /// (default 1).
    #[serde(rename = "quantity_min", default)]
    pub quantity_min: f64,
    /// Priced SKU (alternative to product_id).
    #[serde(rename = "sku", default)]
    pub sku: String,
    #[serde(rename = "unit", default)]
    pub unit: String,
    /// Per-unit price (default 0).
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
    /// Per-entry validity start (promo prices).
    #[serde(rename = "valid_from", default)]
    pub valid_from: String,
    /// Per-entry validity end.
    #[serde(rename = "valid_until", default)]
    pub valid_until: String,
}
