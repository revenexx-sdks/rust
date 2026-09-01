use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderListToCartResult {
    /// Positions written to the cart. Equal to the list's position count minus
    /// `skipped`.
    #[serde(rename = "added", default)]
    pub added: i64,
    /// True when this call created the cart. A created cart is the owner's CURRENT
    /// cart, because a cart the buyer cannot see is not "added to cart".
    #[serde(rename = "cart_created", default)]
    pub cart_created: bool,
    /// The cart the positions landed in: the one that was passed in, or the one
    /// this call created.
    #[serde(rename = "cart_id", default)]
    pub cart_id: String,
    /// The list that was converted. Unchanged by the call — a conversion reads
    /// the list, it never empties it.
    #[serde(rename = "list_id", default)]
    pub list_id: String,
    /// The mode that was actually applied — the one that was asked for, or the
    /// tenant's 'cart_merge_mode' default when the call named none.
    #[serde(rename = "mode", default)]
    pub mode: String,
    /// Positions left out because the catalogue no longer knows their article.
    /// Only ever non-empty when 'on_missing_article' is 'skip' — 'include'
    /// converts them anyway and 'fail' answers 400 instead.
    #[serde(rename = "skipped", default)]
    pub skipped: Vec<crate::models::OrderListSkippedPosition>,
}
