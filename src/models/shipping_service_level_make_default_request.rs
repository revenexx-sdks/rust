use serde::{Deserialize, Serialize};

/// No payload — send {}.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShippingServiceLevelMakeDefaultRequest {
}
