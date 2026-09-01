use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartClaimRequest {
    /// The contact taking ownership. Every active cart of that session ends up
    /// with this contact — adopted as it stands, or folded into
    /// `target_cart_id`.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// The guest session whose active carts are handed over — the key the
    /// storefront keeps in its own session or cookie and has been sending on every
    /// anonymous call. This app neither issues nor parses it, so the example shows
    /// the shape of an opaque token and not a format anything enforces.
    #[serde(rename = "session_key", default)]
    pub session_key: String,
    /// Override the tenant's cart_merge_strategy for this call: 'merge' keeps the
    /// target cart's own lines, 'replace' clears them first. Omit to use the
    /// setting.
    #[serde(rename = "strategy", default)]
    pub strategy: String,
    /// Merge the session carts into this cart instead of adopting them.
    #[serde(rename = "target_cart_id", default)]
    pub target_cart_id: String,
}
