use serde::{Deserialize, Serialize};

/// A position of the placed order — needs an identity: 'name' or 'sku'.
/// Items are SNAPSHOTS: carry the product copy, prices are frozen at
/// place-time.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderItemCreateRequest {
    /// Free-form configuration of configured lines.
    #[serde(rename = "configuration", default)]
    pub configuration: serde_json::Value,
    #[serde(rename = "cost_center", default)]
    pub cost_center: String,
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Falls back to 'sku' when omitted.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Explicit position number; otherwise numbered in steps of the order range's
    /// position_step.
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "position_text", default)]
    pub position_text: String,
    /// Frozen product snapshot at place-time ('snapshot' is accepted as an alias).
    #[serde(rename = "product", default)]
    pub product: serde_json::Value,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Default 1.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// Alias for 'product'.
    #[serde(rename = "snapshot", default)]
    pub snapshot: serde_json::Value,
    /// Derived from line_total and tax_rate when omitted.
    #[serde(rename = "tax_amount", default)]
    pub tax_amount: f64,
    /// Percent (default 0).
    #[serde(rename = "tax_rate", default)]
    pub tax_rate: f64,
    /// Line type (default 'product').
    #[serde(rename = "type", default)]
    pub xtype: String,
    #[serde(rename = "unit", default)]
    pub unit: String,
    /// Per-unit net price — line_total is always derived.
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
    /// Free-form user data.
    #[serde(rename = "user_data", default)]
    pub user_data: serde_json::Value,
}
