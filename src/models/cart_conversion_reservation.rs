use serde::{Deserialize, Serialize};

/// What this app ASKED inventories for, and what it answered. This app holds
/// no stock: inventories picks the location, applies the backorder policy and
/// owns the hold's expiry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartConversionReservation {
    /// Lines inventories accepted without stock behind them, under the tenant's
    /// backorder policy — its policy, not this app's.
    #[serde(rename = "backordered", default)]
    pub backordered: i64,
    /// inventories' hold deadline — its TTL, not this app's.
    #[serde(rename = "expires_at", default)]
    pub expires_at: String,
    /// A hold exists. False with `requested: true` means inventories was asked and
    /// refused — `reason` says why, and only convert_reserves_stock = require
    /// turns that into a 409.
    #[serde(rename = "ok", default)]
    pub ok: bool,
    /// The reference the reservation was booked under: the `order_ref` of the
    /// request, or the cart id when the call carried none. This is the string to
    /// hand inventories when releasing the hold.
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
    /// Why no hold exists — stated, never implied. Present whenever `ok` is
    /// false, and also on the never case.
    #[serde(rename = "reason", default)]
    pub reason: String,
    /// False when convert_reserves_stock is 'never' — no call was made at all,
    /// which is reported rather than dressed up as a silent success.
    #[serde(rename = "requested", default)]
    pub requested: bool,
    /// Lines inventories confirmed a hold for.
    #[serde(rename = "reservations", default)]
    pub reservations: i64,
    /// The HTTP status inventories answered with, present only when it refused.
    /// 404 is its own case: the tenant has no inventories app at all, which is a
    /// different problem from not enough stock.
    #[serde(rename = "status", default)]
    pub status: i64,
}
