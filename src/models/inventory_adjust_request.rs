use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryAdjustRequest {
    /// The corrections, at most 200 in one call — a stocktake, breakage,
    /// shrinkage. Quantities are SIGNED deltas, not new balances.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::InventoryAdjustItem>,
    /// Which location is being corrected. Omitted, the `default_location_code`
    /// setting decides. A correction is per location: the same SKU in two
    /// warehouses is two corrections.
    #[serde(rename = "location_code", default)]
    pub location_code: String,
    /// Inline single-item form: the product to move, instead of a one-entry
    /// `items` array. The two forms are equivalent — nothing downstream knows
    /// which arrived.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Inline single-item form: the SIGNED correction (negative writes stock off,
    /// positive finds it). Non-zero.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// Why the stock is being corrected — this is the audit trail a stocktake
    /// leaves behind. Owed unless `movement_reason_required` is 'none' (its
    /// default, 'adjustments', asks for one exactly here); missing where it is
    /// owed, the call is 400.
    #[serde(rename = "reason", default)]
    pub reason: String,
    /// Inline single-item form: the article number to move (instead of
    /// `product_id`).
    #[serde(rename = "sku", default)]
    pub sku: String,
}
