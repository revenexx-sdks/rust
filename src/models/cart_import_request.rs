use serde::{Deserialize, Serialize};

/// Import into an existing cart ('target_cart_id') or a new cart (owner
/// 'contact_id'/'session_key' required).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartImportRequest {
    /// Owner of the cart this import creates. Ignored when target_cart_id is sent.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// The CSV rows, when that is easier than putting them in `payload`. First
    /// line is the header, and its names are the ones the profile's mapping
    /// expects (the bundled quick-order template reads sku, name, quantity,
    /// unit_price). Numbers are coerced; a JSON column survives as a JSON string.
    #[serde(rename = "csv", default)]
    pub csv: String,
    /// Name for the cart this import creates. A name in the payload's own `cart`
    /// block wins over it; without either the cart is called 'Imported cart'.
    #[serde(rename = "name", default)]
    pub name: String,
    /// The import itself. As an object: `{ "cart": { name, status, currency,
    /// channel_id, metadata }, "items": [ … ] }` — the same document
    /// carts.export produces, so an export round-trips. As a string: that document
    /// as raw JSON, or CSV rows when the profile is a csv one. A line with neither
    /// `name` nor `sku` is dropped, and a payload that leaves no line at all is a
    /// 400.
    #[serde(rename = "payload", default)]
    pub payload: serde_json::Value,
    /// The import profile to run — one of the ids `GET
    /// /carts/io/profiles?direction=import` lists. Omit it for an ad-hoc import:
    /// the payload is then read in the canonical shape, and as CSV if `csv` is
    /// what carried it.
    #[serde(rename = "profile_id", default)]
    pub profile_id: String,
    /// Guest owner of the cart this import creates — the storefront's own
    /// session key. Ignored when target_cart_id is sent.
    #[serde(rename = "session_key", default)]
    pub session_key: String,
    /// An existing ACTIVE cart to import into. The lines are added to it (merging
    /// identical product lines), unless the profile says `apply_mode: replace`,
    /// which clears it first. Without this a new cart is created and an owner is
    /// required.
    #[serde(rename = "target_cart_id", default)]
    pub target_cart_id: String,
}
