use serde::{Deserialize, Serialize};

/// One entry, before and after — the row a confirmation dialog shows.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceAdjustPreviewRow {
    /// The price entry this row is about.
    #[serde(rename = "id", default)]
    pub id: String,
    /// After rounding and ending snapping, in the same currency and on the same
    /// basis. Never negative: below the lowest candidate ending it clamps to it.
    #[serde(rename = "new_unit_price", default)]
    pub new_unit_price: f64,
    /// The product it prices — null when the entry is identified by SKU.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Which rung of the ladder this is.
    #[serde(rename = "quantity_min", default)]
    pub quantity_min: f64,
    /// The SKU it prices — null when the entry is identified by product id.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// Before the change, in the list’s currency and on its tax basis.
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
}
