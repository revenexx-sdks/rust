use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationFailedResponse {
    #[serde(rename = "errors", default)]
    pub errors: Vec<String>,
    #[serde(rename = "status", default)]
    pub status: String,
}
