use serde::{Deserialize, Serialize};

/// Tax resolution status of this answer. resolved=false ⇒ tax_class/tax_rate
/// are unknown, NOT zero.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceTaxContext {
    /// The market whose tax classes were applied.
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    /// Human-readable form of `reason`, in English. Safe to log; not phrased for a
    /// buyer.
    #[serde(rename = "message", default)]
    pub message: String,
    /// Only when resolved=false — why no rate could be applied.
    #[serde(rename = "reason", default)]
    pub reason: String,
    /// true ⇒ every priced item carries `tax_class`, `tax_rate`,
    /// `unit_price_net` and `unit_price_gross`. false ⇒ those are null because
    /// the rate could not be established — read `reason`, and never as "no tax
    /// due".
    #[serde(rename = "resolved", default)]
    pub resolved: bool,
    /// Where the market came from: 'request' (market_id), 'header'
    /// (x-revenexx-market) or 'sole_market' (the tenant has exactly one).
    #[serde(rename = "source", default)]
    pub source: String,
}
