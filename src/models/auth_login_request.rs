use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthLoginRequest {
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "password", default)]
    pub password: String,
}
