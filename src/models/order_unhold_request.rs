use serde::{Deserialize, Serialize};

/// No payload — releasing the hold is a pure state transition, and it clears
/// hold_reason with it. Send {}.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderUnholdRequest {
}
