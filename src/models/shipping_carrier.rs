use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingCarrier {
    /// Stable carrier code, unique per tenant (e.g. dhl, dpd, gls). A method whose
    /// `carrier` text equals this code resolves to this carrier — that is the
    /// migration path off the free-text field. Deliberately no slug pattern: the
    /// column asks only for a non-empty string, and a contract stricter than the
    /// implementation would refuse codes merchants already keep.
    #[serde(rename = "code", default)]
    pub code: String,
    /// The countries this carrier serves. ISO 3166-1 alpha-2 codes; null or an
    /// empty array means no restriction. Compared upper-cased, so a lower-case
    /// entry still matches. Declared as an array rather than the bare object a
    /// jsonb column derives to — this one is always a list.
    #[serde(rename = "countries", default)]
    pub countries: Vec<String>,
    /// When the row was created (UTC).
    #[serde(rename = "created_at", default)]
    pub created_at: String,
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
    /// Row id, assigned by the database on insert.
    #[serde(rename = "id", default)]
    pub id: String,
    /// Localized display names. A flat map keyed by locale — the Cockpit falls
    /// back to `en`. Null means the row has no translations and every client shows
    /// the untranslated column instead.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Free-form jsonb the platform never reads or validates — whatever the
    /// merchant or their integration needs to keep beside the row (a customer
    /// number with the carrier, an ERP key, a label-printer id). The shape varies
    /// BY INTEGRATION, not by anything this app knows, so no key is declared and
    /// none is reserved; the example is one plausible instance rather than a
    /// schema. A flat map of scalars is the convention, and nothing enforces it.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Display name, as an operator typed it.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sort order among the carriers; ties fall back to whatever the database
    /// returns.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The class of service this row represents (default 'standard'), as a CODE
    /// into the tenant's own service levels (GET /shipping/service-levels). One
    /// row is one class: a carrier selling both a parcel and an express product is
    /// two rows. Deliberately not an enum here — the set is the merchant's, so a
    /// fixed list in this contract would make the gateway reject a level they
    /// created. A code the tenant does not keep is a 400 naming the codes they do.
    #[serde(rename = "service_level", default)]
    pub service_level: String,
    /// Whether this carrier may be quoted (default 'active'). Anything else
    /// excludes every method that ships with it from POST /shipping/rates, with a
    /// reason. Tracking links are NOT gated on it — a retired carrier's old
    /// shipments stay resolvable.
    #[serde(rename = "status", default)]
    pub status: String,
    /// Tracking page URL with {tracking_code} where the number goes; {postal_code}
    /// and {country} are also substituted, URL-encoded. Null for a carrier with no
    /// public tracking page.
    #[serde(rename = "tracking_url_template", default)]
    pub tracking_url_template: String,
    /// When the row was last written (UTC).
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
