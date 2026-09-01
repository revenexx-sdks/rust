use serde::{Deserialize, Serialize};

/// The buyer context the checkout resolves rates for — matrix methods need
/// their measure (weight, quantity, order value or attribute) to apply.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingRatesRequest {
    /// The instant to evaluate the delivery estimate at (ISO 8601). Omitted: now.
    /// Lets a storefront compute the cut-off in its own timezone.
    #[serde(rename = "at", default)]
    pub at: String,
    /// Measure values for attribute matrices, keyed by attribute NAME — the key
    /// a matrix method names in its matrix_attribute, and the value the number its
    /// tiers are matched against. Summed over the basket by the caller, not by
    /// this app. Only the key a method asks for is read; anything else in the map
    /// is carried along and ignored, and a value that is not a finite number
    /// excludes that method with a reason rather than failing the quote.
    #[serde(rename = "attributes", default)]
    pub attributes: serde_json::Value,
    /// Destination ISO 3166-1 alpha-2 code — compared upper-cased against method
    /// and carrier country restrictions. Omitted or null: every method that
    /// restricts by country is excluded, with a reason.
    #[serde(rename = "country", default)]
    pub country: String,
    /// ISO 4217 code, echoed into the rates (default 'EUR'). Echoed, not
    /// converted: this app prices in the currency the method carries.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// Buyer market for tax resolution. Omitted: the market matching `country`,
    /// else the tenant's sole market — never an arbitrary one.
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    /// Order value (default 0) — drives order_value matrices, and free-above
    /// thresholds when no sided value is sent. Read on the basis the tenant's
    /// free_above_compares setting declares.
    #[serde(rename = "order_value", default)]
    pub order_value: f64,
    /// Order value including tax. Compared against free-above thresholds when
    /// free_above_compares is 'gross'.
    #[serde(rename = "order_value_gross", default)]
    pub order_value_gross: f64,
    /// Order value excluding tax. Compared against free-above thresholds when
    /// free_above_compares is 'net'.
    #[serde(rename = "order_value_net", default)]
    pub order_value_net: f64,
    /// Total quantity — measure for quantity matrices.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// Total weight — measure for weight matrices. Read in weight_unit and
    /// converted to the unit the tiers are keyed in.
    #[serde(rename = "weight", default)]
    pub weight: f64,
    /// The unit `weight` is expressed in, as a CODE into the tenant's own weight
    /// units (GET /shipping/weight-units). Omitted, it is the unit this market
    /// quotes in. A unit the tenant does not keep is a 400 — a mis-read weight
    /// prices the wrong bracket silently, and guessing is worse than refusing.
    #[serde(rename = "weight_unit", default)]
    pub weight_unit: String,
}
