use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingRate {
    #[serde(rename = "carrier", default)]
    pub carrier: String,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "currency", default)]
    pub currency: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "eta_days_max", default)]
    pub eta_days_max: i64,
    #[serde(rename = "eta_days_min", default)]
    pub eta_days_min: i64,
    #[serde(rename = "free_reason", default)]
    pub free_reason: String,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "position", default)]
    pub position: i64,
    #[serde(rename = "price", default)]
    pub price: f64,
    #[serde(rename = "pricing_type", default)]
    pub pricing_type: String,
    /// Shipping method tax class (or market default).
    #[serde(rename = "tax_class", default)]
    pub tax_class: String,
    /// Tax rate % from markets.tax_classes for this market + tax_class.
    #[serde(rename = "tax_rate", default)]
    pub tax_rate: f64,
}
