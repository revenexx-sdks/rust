use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthRegisterRequest {
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "first_name", default)]
    pub first_name: String,
    #[serde(rename = "last_name", default)]
    pub last_name: String,
    /// BCP 47, e.g. de-DE
    #[serde(rename = "locale", default)]
    pub locale: String,
    /// Join an existing organization.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    /// Found a new organization; the contact becomes its admin.
    #[serde(rename = "organization_name", default)]
    pub organization_name: String,
    #[serde(rename = "password", default)]
    pub password: String,
}
