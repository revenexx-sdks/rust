use serde::{Deserialize, Serialize};

/// An item needs an identity: 'name' or 'sku'.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartItemCreateRequest {
    /// Free-form configuration — configured lines never merge.
    #[serde(rename = "configuration", default)]
    pub configuration: serde_json::Value,
    /// Defaults to the cart's currency.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Falls back to 'sku' when omitted.
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Default 1.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// Loose product snapshot at add-time (price, name, image, …).
    #[serde(rename = "snapshot", default)]
    pub snapshot: serde_json::Value,
    #[serde(rename = "tax_rate", default)]
    pub tax_rate: f64,
    /// Line type (default 'product'). Plain product lines merge by product+price;
    /// configurations always stand alone.
    #[serde(rename = "type", default)]
    pub xtype: String,
    #[serde(rename = "unit", default)]
    pub unit: String,
    /// Per-unit net price — line_total is always derived.
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
}
