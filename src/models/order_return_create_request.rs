use serde::{Deserialize, Serialize};

/// Register a return against the shipped quantities — the return number is
/// drawn from the return range. Omitted positions = every position that still
/// has a returnable quantity, in full ('the customer sent it all back').
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderReturnCreateRequest {
    /// Free-form data for the caller — the returns portal's own reference.
    /// Stored and returned untouched.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// What is coming back. Omitted = every position with a returnable (shipped,
    /// not yet returned) quantity, in full.
    #[serde(rename = "positions", default)]
    pub positions: Vec<crate::models::OrderReturnPosition>,
    /// Why the goods are coming back, free text as the customer or the desk stated
    /// it. Also what /reject stores when it is given no resolution out of the
    /// published set.
    #[serde(rename = "reason", default)]
    pub reason: String,
    /// The default restock flag for positions that carry none of their own — and
    /// the only way to say "put it all back into stock" when the positions are
    /// defaulted. It does not restock anything itself: it decides what the
    /// completion REPORTS for the orchestrator's inventories.restock call.
    #[serde(rename = "restock", default)]
    pub restock: bool,
}
