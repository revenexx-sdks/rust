use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnauthenticatedResponse {
    #[serde(rename = "message", default)]
    pub message: String,
}
