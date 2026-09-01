use serde::{Deserialize, Serialize};

/// One offerable shipping method with its computed price for this buyer
/// context.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingRate {
    /// The carrier CODE — unchanged for every caller that already reads it. The
    /// method's carrier_id, else its `carrier` text, else the tenant's
    /// default_carrier.
    #[serde(rename = "carrier", default)]
    pub carrier: String,
    /// The carrier row's display name, or null when the code names no maintained
    /// carrier.
    #[serde(rename = "carrier_name", default)]
    pub carrier_name: String,
    /// The class of service this rate is, from the carrier row — a code into the
    /// tenant's service levels.
    #[serde(rename = "carrier_service_level", default)]
    pub carrier_service_level: String,
    /// Which step of the chain answered: 'method' (carrier_id), 'method_code' (the
    /// method's text matched a carrier), 'method_text' (it matched none),
    /// 'tenant_default' / 'tenant_default_text' (the setting, matched or not).
    #[serde(rename = "carrier_source", default)]
    pub carrier_source: String,
    /// Stable method code, unique per tenant (e.g. standard, express). What a
    /// checkout and an order line store, so it is the value every integration
    /// joins on.
    #[serde(rename = "code", default)]
    pub code: String,
    /// ISO 4217 code (default EUR). Exactly three characters — the column says
    /// so. Echoed into a rate, never converted: this app prices in the currency
    /// the method carries.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// The delivery window a checkout can print. Calendar days, cut-off evaluated
    /// in UTC (send `at` to control the instant).
    #[serde(rename = "delivery", default)]
    pub delivery: crate::models::ShippingDeliveryEstimate,
    /// The sentence under the name in the checkout — the delivery promise in
    /// words. Null when the name says enough.
    #[serde(rename = "description", default)]
    pub description: String,
    /// Transit time upper bound in calendar days, as applied: the method's own,
    /// else the carrier's.
    #[serde(rename = "eta_days_max", default)]
    pub eta_days_max: i64,
    /// Transit time lower bound in calendar days, as applied: the method's own,
    /// else the carrier's.
    #[serde(rename = "eta_days_min", default)]
    pub eta_days_min: i64,
    /// Only when a free-above threshold applied. Names the compared value AND its
    /// basis (net or gross), and says whether the threshold was the method's own
    /// or shop-wide — the free-shipping promise is a common dispute and this is
    /// the sentence that settles it.
    #[serde(rename = "free_reason", default)]
    pub free_reason: String,
    /// Localized display names. A flat map keyed by locale — the Cockpit falls
    /// back to `en`. Null means the row has no translations and every client shows
    /// the untranslated column instead.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Display name shown in the checkout.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sort order in the checkout (default 0) — a rate answer is returned in
    /// this order.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The shipping fee for this basket, in `currency`, rounded to two decimals
    /// — 0 when a free-above threshold or a 'free' method applied. NULL when
    /// `quote_required` is true: the price is unknown, not zero, and a checkout
    /// must not add 0.00 for it.
    #[serde(rename = "price", default)]
    pub price: f64,
    /// Pricing model (default 'fixed'): 'fixed' is one price for every basket,
    /// 'free' is no price at all, 'matrix' is a tiered price read off this
    /// method's rate tiers. Only 'matrix' looks at matrix_basis, quote_above and
    /// the tier table.
    #[serde(rename = "pricing_type", default)]
    pub pricing_type: String,
    /// Only when quote_required — the measure and the threshold it exceeded, so
    /// an operator pricing it by hand can see what triggered the referral.
    #[serde(rename = "quote_reason", default)]
    pub quote_reason: String,
    /// True when the matrix measure is above the method's quote_above threshold:
    /// the method is still offered, carries no price, and the storefront shows
    /// 'shipping on request'. The order is placed without a computed shipping fee.
    #[serde(rename = "quote_required", default)]
    pub quote_required: bool,
    /// The tax class this rate was taxed under, as a code in markets.tax_classes
    /// — the method's own, the tenant's shipping_tax_class, or the market's
    /// default, whichever answered. Null means unresolved, not untaxed.
    #[serde(rename = "tax_class", default)]
    pub tax_class: String,
    /// The rate in percent from markets.tax_classes for this market and tax_class
    /// — 19 means 19 %. Null means UNKNOWN, never 0: read `tax.resolved` before
    /// treating a missing rate as tax-free.
    #[serde(rename = "tax_rate", default)]
    pub tax_rate: f64,
    /// Which step of the chain supplied the rate: the method's own class, the
    /// tenant's shipping_tax_class, the market default, or the tenant's
    /// default_shipping_tax_rate. Null means unknown, NOT untaxed.
    #[serde(rename = "tax_source", default)]
    pub tax_source: String,
}
