use serde::{Deserialize, Serialize};

/// A position left out of the conversion because the catalogue no longer knows
/// its article (only ever non-empty when the tenant's 'on_missing_article'
/// setting is 'skip').
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderListSkippedPosition {
    /// The position that was left out, so a client can point at the row in the
    /// list.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The saved article name, so the omission can be reported to the buyer in
    /// words they recognise.
    #[serde(rename = "name", default)]
    pub name: String,
    /// The catalogue product the position named, if it named one.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// The article number the position named, if it named one.
    #[serde(rename = "sku", default)]
    pub sku: String,
}
