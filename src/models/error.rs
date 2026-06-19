use serde::{Deserialize, Serialize};

/// Uniform gateway error response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "error", default)]
    pub error: bool,
    #[serde(rename = "message", default)]
    pub message: String,
}
