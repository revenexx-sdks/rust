use serde::{Deserialize, Serialize};

/// What the change did (or would do, on a dry run), plus the rounding policy
/// it was computed under — so a dialog can show a merchant the before/after
/// before it commits.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceEntriesAdjustResponse {
    /// Echo of the request: true means nothing was written.
    #[serde(rename = "dry_run", default)]
    pub dry_run: bool,
    /// Priced entries the filter selected. On-request entries are never counted
    /// — a percentage of "ask us" is not a number.
    #[serde(rename = "matched", default)]
    pub matched: i64,
    /// Decimals the new prices were rounded to before snapping — the tenant’s
    /// price_precision.
    #[serde(rename = "precision", default)]
    pub precision: i64,
    /// The first 50 changes, before and after. `matched` says how many there were
    /// in total.
    #[serde(rename = "preview", default)]
    pub preview: Vec<crate::models::PriceAdjustPreviewRow>,
    /// true when more than 50 entries changed, so `preview` is a sample rather
    /// than the whole set.
    #[serde(rename = "preview_truncated", default)]
    pub preview_truncated: bool,
    /// The price list this answer came out of — enough to link to it or to
    /// explain the number to a merchant ("this came from the dealer list").
    #[serde(rename = "price_list", default)]
    pub price_list: crate::models::PriceListRef,
    /// The price ending the results were snapped to — the request’s, or the
    /// tenant’s bulk_adjust_rounding where it sent none.
    #[serde(rename = "rounding", default)]
    pub rounding: String,
    /// How they landed on the last decimal — the tenant’s rounding_mode.
    #[serde(rename = "rounding_mode", default)]
    pub rounding_mode: String,
    /// Rows actually written — 0 on a dry run, and a price that came out
    /// unchanged is not rewritten.
    #[serde(rename = "updated", default)]
    pub updated: i64,
}
