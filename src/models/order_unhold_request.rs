use serde::{Deserialize, Serialize};

/// No payload — releasing the hold is a pure state transition.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderUnholdRequest {
}
