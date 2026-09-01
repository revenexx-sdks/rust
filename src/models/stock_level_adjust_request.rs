use serde::{Deserialize, Serialize};

/// Correct ONE stock row. The row already knows its location and its item, so
/// a caller owes only the signed delta and a reason — which is exactly what
/// an operator can be asked for in a dialog.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StockLevelAdjustRequest {
    /// The SIGNED correction to this row's `on_hand`: −3 writes off three, +3
    /// finds three. A delta, not the new balance. Zero is refused (400). A
    /// correction that would take `on_hand` below zero is a 422 the database
    /// insists on; one that would take it below this row's own `reserved` is a 422
    /// the `allow_negative_stock` setting can permit.
    #[serde(rename = "quantity", default)]
    pub quantity: f64,
    /// Why this row is being corrected, written onto the ledger booking. Owed
    /// unless `movement_reason_required` is 'none'.
    #[serde(rename = "reason", default)]
    pub reason: String,
}
