use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Contact {
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "external_user_id", default)]
    pub external_user_id: String,
    #[serde(rename = "first_name", default)]
    pub first_name: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "is_primary", default)]
    pub is_primary: bool,
    #[serde(rename = "last_name", default)]
    pub last_name: String,
    #[serde(rename = "locale", default)]
    pub locale: String,
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    #[serde(rename = "phone", default)]
    pub phone: String,
    #[serde(rename = "role", default)]
    pub role: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
