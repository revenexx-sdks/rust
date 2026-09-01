use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartExport {
    /// The export itself. For json: `{ "cart": { name, status, currency,
    /// channel_id, item_count, subtotal }, "items": [ … ] }` — exactly what
    /// carts.import takes back, so an export round-trips. For csv: the lines as a
    /// CSV string, header first, with jsonb columns serialized as JSON text.
    /// Deliberately untyped, because a profile's mapping renames the columns and
    /// that mapping is the caller's own.
    #[serde(rename = "content", default)]
    pub content: String,
    /// A suggested download name, built as `cart-<cart id>.<format>`. Nothing is
    /// stored under it; it is there so a browser download has a name that says
    /// which cart it is.
    #[serde(rename = "filename", default)]
    pub filename: String,
    /// The format that ran — the profile's, or the ad-hoc one.
    #[serde(rename = "format", default)]
    pub format: String,
}
