use serde::{Deserialize, Serialize};

/// What in this app still points at a market tax class, by code.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingTaxClassUsage {
    /// The tax-class code that was asked about, echoed back.
    #[serde(rename = "code", default)]
    pub code: String,
    /// True when this market's shipping_tax_class setting names the code — the
    /// class every method that names none falls back to.
    #[serde(rename = "fallback_setting", default)]
    pub fallback_setting: bool,
    /// True when at least one method or the market fallback setting names it. The
    /// single field a caller deciding whether to allow a delete needs; the rest is
    /// so it can word the refusal.
    #[serde(rename = "in_use", default)]
    pub in_use: bool,
    /// The first 20 of them, so a refusal can name names instead of a number.
    #[serde(rename = "methods", default)]
    pub methods: Vec<serde_json::Value>,
    /// How many methods name this code as their own tax_class. Capped at 500 — a
    /// tenant with more shipping methods than that has a bigger problem than an
    /// imprecise count.
    #[serde(rename = "shipping_methods", default)]
    pub shipping_methods: i64,
}
