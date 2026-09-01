use serde::{Deserialize, Serialize};

/// The exact-column filters this call was understood to carry, verbatim as
/// they arrived. A query parameter that is not a column of `reservations` —
/// a typo, a filter another entity has, `?q=` — is DROPPED and cannot appear
/// here, and the list comes back unfiltered. This object is the only way to
/// tell that apart from "nothing matched".
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReservationsFilter {
    /// The literal `?created_at=` value this call was understood to carry.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The literal `?expires_at=` value this call was understood to carry.
    #[serde(rename = "expires_at", default)]
    pub expires_at: String,
    /// The literal `?id=` value this call was understood to carry.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The literal `?location_id=` value this call was understood to carry.
    #[serde(rename = "location_id", default)]
    pub location_id: String,
    /// The literal `?metadata=` value this call was understood to carry.
    #[serde(rename = "metadata", default)]
    pub metadata: String,
    /// The literal `?order_ref=` value this call was understood to carry.
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
    /// The literal `?product_id=` value this call was understood to carry.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// The literal `?quantity=` value this call was understood to carry.
    #[serde(rename = "quantity", default)]
    pub quantity: String,
    /// The literal `?sku=` value this call was understood to carry.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// The literal `?status=` value this call was understood to carry.
    #[serde(rename = "status", default)]
    pub status: String,
    /// The literal `?updated_at=` value this call was understood to carry.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
