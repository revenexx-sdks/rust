use serde::{Deserialize, Serialize};

/// Tax resolution status of this answer. resolved=false ⇒ tax_class/tax_rate
/// are unknown, NOT zero.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingTaxContext {
    /// The market whose tax classes were applied.
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    /// Human-readable form of `reason`, safe to log or show an operator. One
    /// sentence per reason; the example is the `no_markets` wording.
    #[serde(rename = "message", default)]
    pub message: String,
    /// Only when resolved=false — why no rate could be applied.
    #[serde(rename = "reason", default)]
    pub reason: String,
    /// Whether a tax rate could be applied at all. FALSE means every rate's
    /// tax_class and tax_rate are UNKNOWN — not zero, and not tax-free. A
    /// checkout that adds 0 % on this is wrong; read `reason` and either ask for a
    /// market or refuse to quote.
    #[serde(rename = "resolved", default)]
    pub resolved: bool,
    /// Where the market came from: 'request' (market_id), 'header'
    /// (x-revenexx-market), 'country' (the market matching the destination) or
    /// 'sole_market' (the tenant has exactly one).
    #[serde(rename = "source", default)]
    pub source: String,
    /// Present when the market is known but registers no tax classes and the
    /// tenant's default_shipping_tax_rate supplied the number instead.
    #[serde(rename = "via", default)]
    pub via: String,
}
