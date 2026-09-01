use serde::{Deserialize, Serialize};

/// The generated ladder as stored, plus the rounding policy that shaped it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceEntriesLadderResponse {
    /// The generated rungs, one per requested quantity, ascending — this IS the
    /// item's ladder in this list.
    #[serde(rename = "entries", default)]
    pub entries: Vec<crate::models::PriceEntry>,
    /// Decimals each tier was rounded to before snapping — the tenant's
    /// price_precision.
    #[serde(rename = "precision", default)]
    pub precision: i64,
    /// true when the item's existing entries in this list were removed first (the
    /// default), so the answer is the whole ladder rather than an addition to one.
    #[serde(rename = "replaced", default)]
    pub replaced: bool,
    /// The price ending each tier was snapped to — the request's, or the
    /// tenant's bulk_adjust_rounding.
    #[serde(rename = "rounding", default)]
    pub rounding: String,
    /// How they landed on the last decimal — the tenant's rounding_mode.
    #[serde(rename = "rounding_mode", default)]
    pub rounding_mode: String,
}
