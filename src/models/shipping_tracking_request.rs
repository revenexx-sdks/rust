use serde::{Deserialize, Serialize};

/// One parcel, resolved into a tracking link by the carrier that owns the URL
/// format.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingTrackingRequest {
    /// Carrier code (what an order shipment already stores) or the carrier row id
    /// — a value matching the uuid form is read as the id, anything else as a
    /// code, case-insensitively. Must name a carrier THIS tenant keeps; one that
    /// does not is a 404.
    #[serde(rename = "carrier", default)]
    pub carrier: String,
    /// Destination ISO 3166-1 alpha-2 code — only needed by a template that
    /// names {country}. Upper-cased before substitution.
    #[serde(rename = "country", default)]
    pub country: String,
    /// Destination postcode — only needed by a template that names
    /// {postal_code}.
    #[serde(rename = "postal_code", default)]
    pub postal_code: String,
    /// The carrier's tracking number. Required by every template that names
    /// {tracking_code}, which is all of them in the shipped catalog. URL-encoded
    /// before substitution, so a code with a space or a slash cannot reshape the
    /// link.
    #[serde(rename = "tracking_code", default)]
    pub tracking_code: String,
}
