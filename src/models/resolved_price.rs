use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResolvedPrice {
    #[serde(rename = "currency", default)]
    pub currency: String,
    #[serde(rename = "line_total", default)]
    pub line_total: f64,
    /// true = no price for this buyer context — show "price on request", never
    /// 0.
    #[serde(rename = "on_request", default)]
    pub on_request: bool,
    #[serde(rename = "price_list", default)]
    pub price_list: serde_json::Value,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// Resolved tax class code (from the product, or the market default).
    #[serde(rename = "tax_class", default)]
    pub tax_class: String,
    #[serde(rename = "tax_included", default)]
    pub tax_included: bool,
    /// Tax rate % from markets.tax_classes for this market + tax_class.
    #[serde(rename = "tax_rate", default)]
    pub tax_rate: f64,
    #[serde(rename = "tiers", default)]
    pub tiers: Vec<serde_json::Value>,
    /// Stored price as-is (net or gross per tax_included). Prefer
    /// unit_price_net/unit_price_gross.
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
    /// Gross unit price (incl. tax).
    #[serde(rename = "unit_price_gross", default)]
    pub unit_price_gross: f64,
    /// Net unit price (excl. tax).
    #[serde(rename = "unit_price_net", default)]
    pub unit_price_net: f64,
}
