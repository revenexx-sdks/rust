use serde::{Deserialize, Serialize};

/// No payload — receiving is a pure state transition (registered →
/// received).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderReturnReceiveRequest {
}
