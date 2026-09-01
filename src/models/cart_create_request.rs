use serde::{Deserialize, Serialize};

/// A cart needs an owner: 'contact_id' (customer) or 'session_key' (guest).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartCreateRequest {
    /// The sales channel this cart is being opened in, as a channel of the
    /// channels app. Stored for attribution; nothing in this app reads it.
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    /// The customer who owns this cart, as a contact of the customers app. Send
    /// this OR session_key — a cart with neither owner is refused.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// ISO 4217 code the cart is priced in (default EUR). Lines added without a
    /// currency inherit it.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// Make this THE current cart of its owner as it is created — the same thing
    /// carts.activate does later, and it clears the flag on every sibling cart of
    /// the same owner.
    #[serde(rename = "is_current", default)]
    pub is_current: bool,
    /// Free-form data the storefront hangs on the cart. Stored and returned
    /// verbatim; no key in here is read by this app, and none is indexed.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// What the buyer calls this cart (default 'Cart'). An empty string is legal
    /// and lands on the default.
    #[serde(rename = "name", default)]
    pub name: String,
    /// The guest session that owns this cart — the key the storefront already
    /// keeps in its own session or cookie. Any non-empty string is accepted; this
    /// app issues none and parses none, so the example shows a shape and not a
    /// format. Send this OR contact_id.
    #[serde(rename = "session_key", default)]
    pub session_key: String,
}
