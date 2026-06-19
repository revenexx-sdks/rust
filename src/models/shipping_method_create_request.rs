use serde::{Deserialize, Serialize};

/// A new shipping method — fixed, free or matrix pricing.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingMethodCreateRequest {
    /// Carrier anchor for the upcoming carrier connect (dynamic rates, tracking
    /// links).
    #[serde(rename = "carrier", default)]
    pub carrier: String,
    /// Stable method code, unique per tenant (e.g. standard, express).
    #[serde(rename = "code", default)]
    pub code: String,
    /// Allowed ISO 3166-1 alpha-2 codes; null or empty = worldwide.
    #[serde(rename = "countries", default)]
    pub countries: Vec<String>,
    /// ISO 4217 code (default EUR).
    #[serde(rename = "currency", default)]
    pub currency: String,
    #[serde(rename = "description", default)]
    pub description: String,
    /// Only enabled methods appear in rate responses (default false).
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// Delivery-time estimate for the checkout (days, upper bound).
    #[serde(rename = "eta_days_max", default)]
    pub eta_days_max: i64,
    /// Delivery-time estimate for the checkout (days, lower bound).
    #[serde(rename = "eta_days_min", default)]
    pub eta_days_min: i64,
    /// Free shipping at or above this order value — wins over every pricing
    /// model.
    #[serde(rename = "free_above", default)]
    pub free_above: f64,
    /// Localized display names keyed by locale (e.g. {de, en}).
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Attribute name for matrix_basis 'attribute'.
    #[serde(rename = "matrix_attribute", default)]
    pub matrix_attribute: String,
    /// The measure a matrix method prices over; 'attribute' reads matrix_attribute
    /// from the rate request.
    #[serde(rename = "matrix_basis", default)]
    pub matrix_basis: String,
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Display name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sort order in the checkout (default 0).
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The fixed price (default 0) — ignored for 'free' and 'matrix'.
    #[serde(rename = "price", default)]
    pub price: f64,
    /// Pricing model (default 'fixed'): one price, no price, or tiered over a
    /// measure.
    #[serde(rename = "pricing_type", default)]
    pub pricing_type: String,
}
