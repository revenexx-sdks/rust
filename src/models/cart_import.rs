use serde::{Deserialize, Serialize};

/// `cart` is the cart as it now stands, totals already recomputed — the
/// newly created one, or the target with the imported lines folded in.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartImport {
    #[serde(rename = "cart", default)]
    pub cart: crate::models::Cart,
    /// Lines read out of the payload. Identical product lines merge, so the cart
    /// may have gained fewer rows than this.
    #[serde(rename = "imported_lines", default)]
    pub imported_lines: i64,
}
