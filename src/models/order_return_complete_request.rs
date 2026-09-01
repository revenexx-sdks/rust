use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderReturnCompleteRequest {
    /// How the return was settled. Omitted = settled without recording how.
    #[serde(rename = "resolution", default)]
    pub resolution: String,
}
