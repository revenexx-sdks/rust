use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthRegisterResponse {
    #[serde(rename = "contact", default)]
    pub contact: crate::models::Contact,
    #[serde(rename = "user_id", default)]
    pub user_id: String,
}
