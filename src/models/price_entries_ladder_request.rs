use serde::{Deserialize, Serialize};

/// The quantity ladder (Staffelpreise) for ONE item, generated instead of
/// typed: a price at the first tier and a discount compounded per tier.
/// Identify the item with 'product_id' or 'sku'.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceEntriesLadderRequest {
    /// Price for ONE unit at the FIRST tier, in the list’s currency and on the
    /// list’s tax basis — a decimal amount in major units (19.90), never minor
    /// units/cents.
    #[serde(rename = "base_price", default)]
    pub base_price: f64,
    /// Discount applied per tier, COMPOUNDED down the ladder rather than off the
    /// base price: 5 gives 19.90 / 18.91 / 17.96. Default 0.
    #[serde(rename = "discount_percent", default)]
    pub discount_percent: f64,
    /// The item the ladder prices.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Tier thresholds, ascending — an array of numbers or a comma-separated
    /// string ('1, 10, 50'). Duplicates are collapsed and the set is sorted.
    /// Default [1, 10, 50], at most 50 tiers.
    #[serde(rename = "quantities", default)]
    pub quantities: Vec<f64>,
    /// Default true: the item's existing entries in this list are removed first,
    /// so the ladder IS the ladder. false appends.
    #[serde(rename = "replace", default)]
    pub replace: bool,
    /// Ending the computed prices snap to (nearest match). Omit to use the
    /// tenant's bulk_adjust_rounding setting.
    #[serde(rename = "rounding", default)]
    pub rounding: String,
    /// The item the ladder prices (alternative to product_id).
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// Unit of measure carried onto every generated tier. Free text, neither
    /// validated nor converted.
    #[serde(rename = "unit", default)]
    pub unit: String,
}
