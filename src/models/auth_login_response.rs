use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthLoginResponse {
    #[serde(rename = "contact", default)]
    pub contact: crate::models::Contact,
    #[serde(rename = "session", default)]
    pub session: crate::models::AuthSession,
}
