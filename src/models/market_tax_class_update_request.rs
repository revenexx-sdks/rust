use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketTaxClassUpdateRequest {
    /// Tax class code, unique per market — the rate bucket a product or a
    /// shipping method is assigned to ('standard', 'reduced', 'zero'). Other apps
    /// name a class by THIS and by nothing else: there is no foreign key behind it
    /// and there cannot be (ADR-0055), which is why the delete route asks the
    /// shipping app what still points at the code before removing it.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The class applied to a line that names none. At most one per market. A
    /// market that stores GROSS prices and marks no default cannot break those
    /// prices back down into net, which is why readiness turns that combination
    /// from a warning into a blocking failure.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Localized display names for storefronts and invoices, keyed by locale: a
    /// flat {locale: label} map, one level deep, string values. The key to write
    /// is the `locale_policy.write` from GET /markets/{id}/context, exactly as for
    /// a market's labels. Null means nothing is translated and `name` is all there
    /// is.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Display name of the rate bucket, in the operator's own language.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sort position among this market's tax classes, ascending, default 0 — and
    /// the tie-break that picks a class when none is flagged default.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// Tax rate in PERCENT, 0–100 (default 0) — 20 means 20 %, not 0.2.
    /// Whether a stored price already contains it is a separate question, answered
    /// per market by `pricing.tax_basis` on the context.
    #[serde(rename = "rate", default)]
    pub rate: f64,
}
