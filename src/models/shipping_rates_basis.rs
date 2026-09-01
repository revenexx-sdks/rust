use serde::{Deserialize, Serialize};

/// How this answer was measured — the tenant settings that shaped it, echoed
/// so the numbers can be re-derived.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingRatesBasis {
    /// The instant the delivery estimates were computed from.
    #[serde(rename = "evaluated_at", default)]
    pub evaluated_at: String,
    /// Whether free-above thresholds were compared against the net or the gross
    /// order value.
    #[serde(rename = "free_above_compares", default)]
    pub free_above_compares: String,
    /// The measure a matrix method without its own basis priced over.
    #[serde(rename = "matrix_basis_default", default)]
    pub matrix_basis_default: String,
    /// The unit the request expressed its weight in; converted to weight_unit
    /// before any tier was matched.
    #[serde(rename = "request_weight_unit", default)]
    pub request_weight_unit: String,
    /// Kilograms per unit of `request_weight_unit`, as applied.
    #[serde(rename = "request_weight_unit_factor", default)]
    pub request_weight_unit_factor: f64,
    /// The unit the rate tiers are keyed in — this market's `weight_unit`
    /// setting, else the unit the tenant flagged as default.
    #[serde(rename = "weight_unit", default)]
    pub weight_unit: String,
    /// Kilograms per unit of `weight_unit`, as applied. Echoed because a unit is a
    /// code PLUS a number and the number is what priced the parcel — a quote has
    /// to be re-derivable from its own payload, not from a table the merchant may
    /// since have edited.
    #[serde(rename = "weight_unit_factor", default)]
    pub weight_unit_factor: f64,
}
