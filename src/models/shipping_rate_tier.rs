use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingRateTier {
    /// When the row was created (UTC).
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Lower bound of this tier, in the method's matrix measure — kilograms (or
    /// whatever the market's `weight_unit` names, converted through its factor)
    /// for a weight matrix, items for quantity, money in the method's currency for
    /// order_value, and the raw attribute value for 'attribute'. INCLUSIVE: the
    /// tier applies from this value upward, and the tier that wins is the one with
    /// the highest from_value at or below the measured value, so a measure of
    /// exactly 10 is priced by the tier at 10 rather than the one below it. The
    /// last tier has no upper bound. Unique per method — a second tier at the
    /// same threshold is a 409, because which of the two won would be whatever the
    /// database returned first.
    #[serde(rename = "from_value", default)]
    pub from_value: f64,
    /// Row id, assigned by the database on insert.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The shipping method this tier prices. Set from the path on every write, so
    /// a body that names another method is ignored rather than obeyed. ON DELETE
    /// CASCADE: deleting the method deletes its table.
    #[serde(rename = "method_id", default)]
    pub method_id: String,
    /// Display order in the matrix editor (default 0; a bulk replace derives it
    /// from the array index). Pricing reads from_value, never this.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// What this tier costs, in the method's currency. Charged in full for the
    /// whole consignment — a matrix is a lookup table, not a rate per unit.
    #[serde(rename = "price", default)]
    pub price: f64,
    /// When the row was last written (UTC).
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
