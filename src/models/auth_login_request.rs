use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthLoginRequest {
    /// The buyer's login address — the same one the contact carries.
    #[serde(rename = "email", default)]
    pub email: String,
    /// The password from registration or recovery. Wrong credentials are a 401; a
    /// correct one on an undecided application is a 403.
    #[serde(rename = "password", default)]
    pub password: String,
}
