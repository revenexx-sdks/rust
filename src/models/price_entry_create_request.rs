use serde::{Deserialize, Serialize};

/// An entry needs an identity: 'product_id' or 'sku'.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceEntryCreateRequest {
    /// Free-form bag: whatever JSON object you write round-trips exactly, and this
    /// app never reads it. Its keys are yours.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Default 'standard'; 'on_request' is the explicit no-price marker — it
    /// STOPS resolution for this item on this list and answers "price on request"
    /// even where a cheaper list exists.
    #[serde(rename = "price_type", default)]
    pub price_type: String,
    /// The product this rung prices. An entry needs product_id or sku — the row
    /// CHECK enforces it.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Tier threshold (Staffelpreis): this price applies from this quantity
    /// upwards (default 1). The rungs of one item are the entries sharing its
    /// identity; the highest threshold at or below the requested quantity wins.
    #[serde(rename = "quantity_min", default)]
    pub quantity_min: f64,
    /// The article number this rung prices (alternative to product_id). Matched
    /// exactly on resolve — never normalised or case-folded.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// Unit of measure the price is per — free text, neither validated nor
    /// converted here. A resolve call’s `quantity` is counted in it.
    #[serde(rename = "unit", default)]
    pub unit: String,
    /// Price for ONE unit of `unit`, in the LIST’s currency and on the LIST’s
    /// tax basis — a decimal amount in major units (19.90), never minor
    /// units/cents. Stored at 4 decimals and echoed back exactly as sent (default
    /// 0).
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
    /// Start of this entry’s own validity (ISO 8601) — how a promo price is
    /// expressed: a second rung, live only for its window. null = open-ended.
    #[serde(rename = "valid_from", default)]
    pub valid_from: String,
    /// End of this entry’s own validity; null = open-ended. Outside it the rung
    /// is skipped and the ladder resolves as if it were not there.
    #[serde(rename = "valid_until", default)]
    pub valid_until: String,
}
