use serde::{Deserialize, Serialize};

/// Change every priced entry of a list at once. Send 'percent' OR 'amount',
/// never both. On-request entries are never touched — a percentage of "ask
/// us" is not a number.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceEntriesAdjustRequest {
    /// Absolute change added to every unit price, in the list's currency.
    #[serde(rename = "amount", default)]
    pub amount: f64,
    /// true writes nothing and answers the same preview — what the Cockpit
    /// dialog shows before it commits.
    #[serde(rename = "dry_run", default)]
    pub dry_run: bool,
    /// Relative change in percent: 5 raises by 5 %, -10 cuts by 10 %.
    #[serde(rename = "percent", default)]
    pub percent: f64,
    /// Ending the computed prices snap to (nearest match). Omit to use the
    /// tenant's bulk_adjust_rounding setting.
    #[serde(rename = "rounding", default)]
    pub rounding: String,
    /// Restrict the change to entries whose SKU starts with this (a prefix,
    /// case-sensitive, no wildcards). Entries identified only by product_id never
    /// match a prefix. Omit to change the whole list.
    #[serde(rename = "sku_prefix", default)]
    pub sku_prefix: String,
}
