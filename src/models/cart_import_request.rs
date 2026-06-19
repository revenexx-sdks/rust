use serde::{Deserialize, Serialize};

/// Import into an existing cart ('target_cart_id') or a new cart (owner
/// 'contact_id'/'session_key' required).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartImportRequest {
    /// Owner of a newly created cart.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// Raw CSV content (alternative to payload for csv profiles).
    #[serde(rename = "csv", default)]
    pub csv: String,
    /// Name for a newly created cart.
    #[serde(rename = "name", default)]
    pub name: String,
    /// The import payload: '{cart, items}' object, or a raw JSON/CSV string in the
    /// profile's format.
    #[serde(rename = "payload", default)]
    pub payload: serde_json::Value,
    /// Import profile to run; ad-hoc import when omitted.
    #[serde(rename = "profile_id", default)]
    pub profile_id: String,
    /// Guest owner of a newly created cart.
    #[serde(rename = "session_key", default)]
    pub session_key: String,
    /// Existing active cart to import into.
    #[serde(rename = "target_cart_id", default)]
    pub target_cart_id: String,
}
