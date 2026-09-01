use serde::{Deserialize, Serialize};

/// Stop the order. The reason is optional but is what the guard quotes back at
/// whoever tries to ship, so an unexplained hold is a hold nobody can resolve.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderHoldRequest {
    /// Why the order is held, in the words the shipping guard quotes back. Null
    /// when it is not held — releasing a hold clears it.
    #[serde(rename = "reason", default)]
    pub reason: String,
}
