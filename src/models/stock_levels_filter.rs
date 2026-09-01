use serde::{Deserialize, Serialize};

/// The exact-column filters this call was understood to carry, verbatim as
/// they arrived. A query parameter that is not a column of `stock_levels` —
/// a typo, a filter another entity has, `?q=` — is DROPPED and cannot appear
/// here, and the list comes back unfiltered. This object is the only way to
/// tell that apart from "nothing matched".
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StockLevelsFilter {
    /// The literal `?created_at=` value this call was understood to carry.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The literal `?id=` value this call was understood to carry.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The literal `?location_id=` value this call was understood to carry.
    #[serde(rename = "location_id", default)]
    pub location_id: String,
    /// The literal `?metadata=` value this call was understood to carry.
    #[serde(rename = "metadata", default)]
    pub metadata: String,
    /// The literal `?on_hand=` value this call was understood to carry.
    #[serde(rename = "on_hand", default)]
    pub on_hand: String,
    /// The literal `?product_id=` value this call was understood to carry.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// The literal `?reorder_point=` value this call was understood to carry.
    #[serde(rename = "reorder_point", default)]
    pub reorder_point: String,
    /// The literal `?reserved=` value this call was understood to carry.
    #[serde(rename = "reserved", default)]
    pub reserved: String,
    /// The literal `?sku=` value this call was understood to carry.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// The literal `?updated_at=` value this call was understood to carry.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
