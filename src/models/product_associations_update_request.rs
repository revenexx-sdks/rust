use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductAssociationsUpdateRequest {
    /// Which kind of relation this is — the `association_types` row.
    #[serde(rename = "association_type_id", default)]
    pub association_type_id: String,
    /// Order in which the targets are shown, ascending.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The product the relation starts at — the one whose detail page shows it.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// How many of the target belong to the source — the 4 in "this bundle
    /// contains 4 casters". Only meaningful when the association type carries
    /// `is_quantified`; null on an ordinary cross-sell.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// The product the relation points at — the accessory, the spare part, the
    /// cross-sell.
    #[serde(rename = "target_product_id", default)]
    pub target_product_id: String,
}
