use serde::{Deserialize, Serialize};

/// One carrier this app knows the facts for, exactly as it would be created.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingCarrierCatalogEntry {
    /// The code the seeded row would carry, and the code a method's `carrier` text
    /// has to match to resolve to it.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The countries this carrier serves. ISO 3166-1 alpha-2 codes; null or an
    /// empty array means no restriction. Compared upper-cased, so a lower-case
    /// entry still matches. Declared as an array rather than the bare object a
    /// jsonb column derives to — this one is always a list.
    #[serde(rename = "countries", default)]
    pub countries: Vec<String>,
    /// This carrier's own daily pickup cut-off, HH:MM in 24-hour form, UTC.
    /// Overrides the tenant's cutoff_time for methods on this carrier — one
    /// shop-wide time cannot be both DHL's 16:00 and a forwarder's 12:00. Null or
    /// the empty string means this carrier declares none; any other shape is a
    /// 400, because a cut-off the estimator cannot read is a delivery promise
    /// silently computed without one.
    #[serde(rename = "cutoff_time", default)]
    pub cutoff_time: String,
    /// Transit time upper bound, in calendar days from the ship date.
    #[serde(rename = "eta_days_max", default)]
    pub eta_days_max: i64,
    /// Transit time lower bound, in calendar days from the ship date — inherited
    /// by any method on this carrier that states no ETA of its own.
    #[serde(rename = "eta_days_min", default)]
    pub eta_days_min: i64,
    /// Days needed to make a consignment ready for THIS carrier, added to the ship
    /// date before the transit days. Overrides the tenant's handling_days.
    #[serde(rename = "handling_days", default)]
    pub handling_days: i64,
    /// Localized display names the seed would carry. A flat map keyed by locale
    /// — the Cockpit falls back to `en`. Null means the row has no translations
    /// and every client shows the untranslated column instead.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// The display name the seeded row would carry. An existing row keeps the
    /// merchant's own name — the seed never writes over one.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Whether a fresh install starts with this carrier. False means this app
    /// knows how to describe it but only creates it when asked.
    #[serde(rename = "seeded", default)]
    pub seeded: bool,
    /// Service-level code the seeded row carries — one of the tenant's own
    /// values.
    #[serde(rename = "service_level", default)]
    pub service_level: String,
    /// Tracking page URL with {tracking_code} where the number goes; {postal_code}
    /// and {country} are also substituted, URL-encoded. Null for a carrier with no
    /// public tracking page.
    #[serde(rename = "tracking_url_template", default)]
    pub tracking_url_template: String,
}
