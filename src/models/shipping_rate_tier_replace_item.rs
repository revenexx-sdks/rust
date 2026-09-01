use serde::{Deserialize, Serialize};

/// A matrix tier of the new set (from_value → price) — null falls back to
/// 0, position derives from the array order.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingRateTierReplaceItem {
    /// Lower bound of this tier, in the method's matrix measure — kilograms (or
    /// whatever the market's `weight_unit` names, converted through its factor)
    /// for a weight matrix, items for quantity, money in the method's currency for
    /// order_value, and the raw attribute value for 'attribute'. INCLUSIVE: the
    /// tier applies from this value upward, and the tier that wins is the one with
    /// the highest from_value at or below the measured value, so a measure of
    /// exactly 10 is priced by the tier at 10 rather than the one below it. The
    /// last tier has no upper bound. Unique per method — a second tier at the
    /// same threshold is a 409, because which of the two won would be whatever the
    /// database returned first. Null falls back to 0.
    #[serde(rename = "from_value", default)]
    pub from_value: f64,
    /// Ignored — derived from the array index.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// What this tier costs, in the method's currency. Charged in full for the
    /// whole consignment — a matrix is a lookup table, not a rate per unit. Null
    /// falls back to 0.
    #[serde(rename = "price", default)]
    pub price: f64,
}
