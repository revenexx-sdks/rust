use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemAvailability {
    /// on_hand − reserved across the locations in scope: available-to-promise,
    /// and the number a storefront shows. It can be NEGATIVE once backorders have
    /// been reserved beyond stock — nothing floors it, because "sold more than
    /// we hold" is a real state a merchant needs to see.
    #[serde(rename = "available", default)]
    pub available: f64,
    /// The per-location breakdown behind the summed figures — which place could
    /// actually ship it.
    #[serde(rename = "locations", default)]
    pub locations: Vec<crate::models::LocationAvailability>,
    /// Physically in stock, summed across the locations in scope (every enabled
    /// location, or the one `location_code` named). Promised units are included,
    /// so this is NOT what may be sold.
    #[serde(rename = "on_hand", default)]
    pub on_hand: f64,
    /// True when the item is tracked and `available >= requested` at this moment.
    /// A SNAPSHOT, not a hold: nothing is set aside until POST
    /// /inventories/reserve, and two checkouts can both read true for the last
    /// unit.
    #[serde(rename = "orderable", default)]
    pub orderable: bool,
    /// The product id as it was asked for, echoed. Null when the item was named by
    /// SKU.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// The quantity the check was made against — the item's own `quantity`, or 1
    /// when none was sent. `orderable` answers "can I have this many?", so it is
    /// only as strict as this number.
    #[serde(rename = "requested", default)]
    pub requested: f64,
    /// Already promised to orders, summed across the same locations — the part
    /// of `on_hand` that is spoken for.
    #[serde(rename = "reserved", default)]
    pub reserved: f64,
    /// The SKU as it was asked for, echoed. Null when the item was named by
    /// product id.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// False when this app has never seen the item: no stock row anywhere in
    /// scope. It is not an error and not a zero — the storefront decides whether
    /// an untracked item sells freely (a service, a made-to-order piece) or not at
    /// all. `on_hand`, `reserved` and `available` are 0 in that case, and
    /// `orderable` is false.
    #[serde(rename = "tracked", default)]
    pub tracked: bool,
}
