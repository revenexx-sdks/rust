use serde::{Deserialize, Serialize};

/// One rung of the winning list’s quantity ladder for this item.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceTier {
    /// The quantity this rung applies from. The rung with the highest
    /// `quantity_min` at or below the requested quantity is the one `unit_price`
    /// on the item was taken from.
    #[serde(rename = "quantity_min", default)]
    pub quantity_min: f64,
    /// Unit of measure the rung’s price is per. Absent when the entry names
    /// none.
    #[serde(rename = "unit", default)]
    pub unit: String,
    /// The rung’s price for ONE unit, in the answer’s `currency` and on the
    /// item’s `tax_basis` — decimal major units, exactly as stored. Tiers are
    /// NOT tax-adjusted: only the chosen price gets
    /// `unit_price_net`/`unit_price_gross`.
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
}
