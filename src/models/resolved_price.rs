use serde::{Deserialize, Serialize};

/// What one item costs this buyer, and which list said so.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResolvedPrice {
    /// ISO 4217 currency of every amount on this item. Always the winning list’s
    /// currency, which always equals the call’s top-level `currency` —
    /// resolution only considers lists that match it, so a list and its answer can
    /// never disagree. null on an on-request item.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// Present ONLY on an item that named neither `product_id` nor `sku`, and
    /// always with this exact text. The call still answers 200 and the item comes
    /// back on_request, because one malformed line must not cost a whole cart its
    /// prices.
    #[serde(rename = "error", default)]
    pub error: String,
    /// `unit_price × quantity`, on the SAME basis as `unit_price` (so net if the
    /// list is net) and rounded to `basis.price_precision`. Not a tax-adjusted
    /// total — a cart computes its own from the net/gross pair.
    #[serde(rename = "line_total", default)]
    pub line_total: f64,
    /// true = no price for this buyer context — show "price on request", never
    /// 0.
    #[serde(rename = "on_request", default)]
    pub on_request: bool,
    /// Why there is no price: nothing prices it, a list marks it on-request, the
    /// tenant hides prices from anonymous buyers, or the item named neither
    /// product_id nor sku.
    #[serde(rename = "on_request_reason", default)]
    pub on_request_reason: String,
    /// The list that priced this item — null when nothing did. On an
    /// `on_request_entry` answer it is the list that said "ask us".
    #[serde(rename = "price_list", default)]
    pub price_list: serde_json::Value,
    /// Echo of the requested `product_id` — null when the item was identified by
    /// SKU.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// The quantity this answer was computed for: what you sent, or 1 where you
    /// sent nothing or a non-positive value. It selects the tier and multiplies
    /// into `line_total`.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// Echo of the requested `sku` — null when the item was identified by
    /// product id.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// Whether the stored amount is net or gross. THE fact a price cannot be
    /// without.
    #[serde(rename = "tax_basis", default)]
    pub tax_basis: String,
    /// Who decided it: the list's own tax_basis, a legacy tax_included=true on the
    /// list, or the tenant's tax_inclusive_default setting.
    #[serde(rename = "tax_basis_source", default)]
    pub tax_basis_source: String,
    /// The tax class code that produced `tax_rate`: the product’s own class
    /// where the products app knows one, otherwise the buyer market’s default
    /// class. The codes are the tenant’s, defined in `markets.tax_classes` —
    /// conventionally `standard` and `reduced`. null when tax could not be
    /// resolved.
    #[serde(rename = "tax_class", default)]
    pub tax_class: String,
    /// Whether unit_price already contains tax. Never null on a priced item — it
    /// is `tax_basis` as a boolean, kept for existing callers.
    #[serde(rename = "tax_included", default)]
    pub tax_included: bool,
    /// Tax rate as a PERCENTAGE (19 means 19 %, not 0.19), read from
    /// `markets.tax_classes` for this market and `tax_class`. null means UNKNOWN
    /// — a checkout must be able to tell that apart from a genuine 0 %.
    #[serde(rename = "tax_rate", default)]
    pub tax_rate: f64,
    /// The FULL quantity ladder the winning list holds for this item, ascending by
    /// `quantity_min` — what a PDP renders as a tier table. Empty on an
    /// on-request item.
    #[serde(rename = "tiers", default)]
    pub tiers: Vec<crate::models::PriceTier>,
    /// Price for ONE unit, in `currency` and on the basis `tax_basis` names — a
    /// decimal amount in major units (19.90 EUR), never minor units/cents. It is
    /// the stored rung exactly as a merchant typed it, unrounded. Do not display
    /// it without reading `tax_basis`; prefer `unit_price_net`/`unit_price_gross`,
    /// which are unambiguous.
    #[serde(rename = "unit_price", default)]
    pub unit_price: f64,
    /// Unit price INCLUDING tax, in `currency`, rounded to `basis.price_precision`
    /// under `basis.rounding_mode`. Derived from `unit_price` and `tax_rate` in
    /// whichever direction `tax_basis` requires. Present only when `tax.resolved`
    /// is true.
    #[serde(rename = "unit_price_gross", default)]
    pub unit_price_gross: f64,
    /// Unit price EXCLUDING tax, in `currency`, rounded to `basis.price_precision`
    /// under `basis.rounding_mode`. Present only when `tax.resolved` is true —
    /// null means the rate is unknown, not that there is no tax.
    #[serde(rename = "unit_price_net", default)]
    pub unit_price_net: f64,
}
