use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingMethodUpdateRequest {
    /// Carrier CODE, kept from before shipping_carriers existed. Looked up in the
    /// carrier table when carrier_id is not set, so an existing value keeps
    /// working and gains a tracking template; a code nobody maintains is still
    /// reported as a plain name.
    #[serde(rename = "carrier", default)]
    pub carrier: String,
    /// The carrier this method ships with. Wins over `carrier` and supplies the
    /// tracking template, pickup cut-off, handling time and transit days.
    #[serde(rename = "carrier_id", default)]
    pub carrier_id: String,
    /// Stable method code, unique per tenant (e.g. standard, express). What a
    /// checkout and an order line store, so it is the value every integration
    /// joins on.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The countries this method may be offered into. ISO 3166-1 alpha-2 codes;
    /// null or an empty array means no restriction. Compared upper-cased, so a
    /// lower-case entry still matches. Declared as an array rather than the bare
    /// object a jsonb column derives to — this one is always a list. ANDed with
    /// the carrier's own reach.
    #[serde(rename = "countries", default)]
    pub countries: Vec<String>,
    /// ISO 4217 code (default EUR). Exactly three characters — the column says
    /// so. Echoed into a rate, never converted: this app prices in the currency
    /// the method carries.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// The sentence under the name in the checkout — the delivery promise in
    /// words. Null when the name says enough.
    #[serde(rename = "description", default)]
    pub description: String,
    /// Only enabled methods are ever quoted (default false); a disabled one is
    /// reported in `excluded` rather than hidden.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// Transit time upper bound in calendar days. Falls back to the carrier's when
    /// null.
    #[serde(rename = "eta_days_max", default)]
    pub eta_days_max: i64,
    /// Transit time lower bound in calendar days, for the checkout. Falls back to
    /// the carrier's when null.
    #[serde(rename = "eta_days_min", default)]
    pub eta_days_min: i64,
    /// Free shipping at or above this order value — wins over every pricing
    /// model, including a matrix. Compared net or gross as the market's
    /// free_above_compares setting declares. Null falls back to the tenant's
    /// shop-wide free_shipping_threshold.
    #[serde(rename = "free_above", default)]
    pub free_above: f64,
    /// Localized display names. A flat map keyed by locale — the Cockpit falls
    /// back to `en`. Null means the row has no translations and every client shows
    /// the untranslated column instead.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Attribute name for matrix_basis 'attribute' — the key the rate request's
    /// `attributes` map is read at. Free text: the set of attributes is the
    /// catalogue's, not this app's.
    #[serde(rename = "matrix_attribute", default)]
    pub matrix_attribute: String,
    /// The measure a matrix method prices its tiers over: total basket weight (in
    /// the market's weight unit), total item count, order value, or 'attribute'
    /// — any number the rate request carries under matrix_attribute. Null falls
    /// back to the tenant's matrix_basis_default. Ignored unless pricing_type is
    /// 'matrix'.
    #[serde(rename = "matrix_basis", default)]
    pub matrix_basis: String,
    /// Free-form jsonb the platform never reads or validates — whatever the
    /// merchant or their integration needs to keep beside the row (a customer
    /// number with the carrier, an ERP key, a label-printer id). The shape varies
    /// BY INTEGRATION, not by anything this app knows, so no key is declared and
    /// none is reserved; the example is one plausible instance rather than a
    /// schema. A flat map of scalars is the convention, and nothing enforces it.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Display name shown in the checkout.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sort order in the checkout (default 0) — a rate answer is returned in
    /// this order.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The fixed price (default 0), in `currency` — ignored for 'free' and
    /// 'matrix'.
    #[serde(rename = "price", default)]
    pub price: f64,
    /// Pricing model (default 'fixed'): 'fixed' is one price for every basket,
    /// 'free' is no price at all, 'matrix' is a tiered price read off this
    /// method's rate tiers. Only 'matrix' looks at matrix_basis, quote_above and
    /// the tier table.
    #[serde(rename = "pricing_type", default)]
    pub pricing_type: String,
    /// Above this MATRIX MEASURE the method carries no automatic price: it is
    /// still offered, flagged `quote_required` with a reason, and the storefront
    /// shows 'shipping on request'. For bulky or overweight freight priced by
    /// hand. Null = every measure is priced automatically.
    #[serde(rename = "quote_above", default)]
    pub quote_above: f64,
    /// This method's own tax class, as a CODE into the buyer market's tax classes
    /// (markets.tax_classes) — never a rate. First step of the tax chain: unset
    /// falls back to the tenant's shipping_tax_class setting, then the market
    /// default. Not a foreign key and it could not be (ADR-0055); GET
    /// /shipping/tax-classes/{code}/usage is the integrity question markets asks
    /// in its place.
    #[serde(rename = "tax_class", default)]
    pub tax_class: String,
}
