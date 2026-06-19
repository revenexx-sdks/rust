use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthMeResponse {
    #[serde(rename = "contact", default)]
    pub contact: crate::models::Contact,
    #[serde(rename = "user", default)]
    pub user: serde_json::Value,
}
