use serde::{Deserialize, Serialize};

/// No payload — send {}. The kind is named by the path, and there is nothing
/// else to decide.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderListKindMakeDefaultRequest {
}
