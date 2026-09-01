use serde::{Deserialize, Serialize};

/// Only safe columns are updatable — status moves through the lifecycle
/// routes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartUpdateRequest {
    /// Move the cart to another sales channel.
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    /// ISO 4217 code. Changes what NEW lines inherit; lines already in the cart
    /// keep the currency they were added with.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// Free-form data the storefront hangs on the cart. Stored and returned
    /// verbatim; no key in here is read by this app, and none is indexed.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Rename the cart. Unlike on create, this is written verbatim — `null` and
    /// `''` are refused by the database.
    #[serde(rename = "name", default)]
    pub name: String,
}
