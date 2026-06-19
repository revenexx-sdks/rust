use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemAvailability {
    #[serde(rename = "available", default)]
    pub available: f64,
    #[serde(rename = "locations", default)]
    pub locations: Vec<serde_json::Value>,
    #[serde(rename = "on_hand", default)]
    pub on_hand: f64,
    #[serde(rename = "orderable", default)]
    pub orderable: bool,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "requested", default)]
    pub requested: f64,
    #[serde(rename = "reserved", default)]
    pub reserved: f64,
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// false = unknown to inventory; the storefront decides whether untracked
    /// items sell freely.
    #[serde(rename = "tracked", default)]
    pub tracked: bool,
}
