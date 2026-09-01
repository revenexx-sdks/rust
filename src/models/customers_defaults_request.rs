use serde::{Deserialize, Serialize};

/// No fields — send {}.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomersDefaultsRequest {
}
