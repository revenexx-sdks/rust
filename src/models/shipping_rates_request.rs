use serde::{Deserialize, Serialize};

/// The buyer context the checkout resolves rates for — matrix methods need
/// their measure (weight, quantity, order value or attribute) to apply.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingRatesRequest {
    /// Measure values for attribute matrices, keyed by attribute name.
    #[serde(rename = "attributes", default)]
    pub attributes: serde_json::Value,
    /// Destination ISO 3166-1 alpha-2 code — checked against method country
    /// restrictions.
    #[serde(rename = "country", default)]
    pub country: String,
    /// Echoed into the rates (default 'EUR').
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// Buyer market for tax resolution (else inferred from country, else first
    /// market).
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    /// Order value (default 0) — drives free-above thresholds and order_value
    /// matrices.
    #[serde(rename = "order_value", default)]
    pub order_value: f64,
    /// Total quantity — measure for quantity matrices.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// Total weight — measure for weight matrices.
    #[serde(rename = "weight", default)]
    pub weight: f64,
}
