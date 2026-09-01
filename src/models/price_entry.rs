use serde::{Deserialize, Serialize};

/// One rung of one item’s quantity ladder inside one price list. The ladder
/// IS the set of entries sharing an identity (product_id or sku); the amount
/// is in the LIST’s currency and on the LIST’s tax basis.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceEntry {
    /// When the entry was created.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The entry itself — one rung of one item’s quantity ladder.
    #[serde(rename = "id", default)]
    pub id: String,
    /// Free-form bag, unvalidated and never read by this app: whatever JSON object
    /// you write round-trips exactly. Its keys are the integration’s own, e.g.
    /// {"source_system": "erp", "imported_batch": "2026-02-14"}.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// The price list this entry belongs to, and therefore the currency and tax
    /// basis its amount is on. Set from the path on write.
    #[serde(rename = "price_list_id", default)]
    pub price_list_id: String,
    /// `standard` is a number. `on_request` is the explicit no-price marker: it
    /// STOPS resolution for this item on this list and answers price-on-request,
    /// even where a cheaper list exists — the list is authoritative for this
    /// buyer and it says "ask us".
    #[serde(rename = "price_type", default)]
    pub price_type: String,
    /// The product this rung prices. An entry needs `product_id` or `sku` (a row
    /// CHECK enforces it); an entry that carries both prices whichever of the two
    /// the resolve item names.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// Lowest quantity this price applies from (Staffelpreis). The ladder for one
    /// item is the set of entries sharing its identity: the rung with the HIGHEST
    /// quantity_min at or below the requested quantity wins, and below the first
    /// rung the first rung’s price applies — a minimum order quantity belongs
    /// to the catalog, not to the ladder.
    #[serde(rename = "quantity_min", default)]
    pub quantity_min: f64,
    /// The article number this rung prices, for a price book keyed by SKU rather
    /// than by product id — matched exactly, never normalised or case-folded.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// The unit of measure the price is per — ‘pcs’, ‘m’, ‘kg’, a
    /// packaging size. Free text: this app neither validates nor converts it, and
    /// the `quantity` of a resolve call is counted in it.
    #[serde(rename = "unit", default)]
    pub unit: String,
    /// Price for ONE unit of `unit`, expressed in the list’s `currency` and on
    /// the list’s `tax_basis` — a decimal amount in major units (19.90 EUR),
    /// never minor units/cents. Stored at 4 decimals so a per-1000-piece price
    /// survives, and echoed back exactly as it was written; only DERIVED amounts
    /// (net, gross, line totals) are rounded to the tenant’s `price_precision`.
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
    /// When the entry last changed. A bulk adjust only writes the rows whose price
    /// actually moved, so this is a real "the price changed here" marker.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    /// Start of this entry’s own validity; null = open-ended. This is how a
    /// promo price is expressed — a second rung for the same item and quantity,
    /// live only for its window.
    #[serde(rename = "valid_from", default)]
    pub valid_from: String,
    /// End of this entry’s own validity; null = open-ended. Outside the window
    /// the rung is skipped and the ladder resolves as if it were not there.
    #[serde(rename = "valid_until", default)]
    pub valid_until: String,
}
