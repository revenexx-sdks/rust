use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthRecoveryConfirmRequest {
    #[serde(rename = "password", default)]
    pub password: String,
    #[serde(rename = "secret", default)]
    pub secret: String,
    #[serde(rename = "user_id", default)]
    pub user_id: String,
}
