use serde::{Deserialize, Serialize};

/// A cart needs an owner: 'contact_id' (customer) or 'session_key' (guest).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartCreateRequest {
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    /// Owning customer contact.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// ISO 4217 code (default EUR).
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// Make this THE current cart of its owner.
    #[serde(rename = "is_current", default)]
    pub is_current: bool,
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Display name (default 'Cart').
    #[serde(rename = "name", default)]
    pub name: String,
    /// Owning guest session.
    #[serde(rename = "session_key", default)]
    pub session_key: String,
}
