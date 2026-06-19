use serde::{Deserialize, Serialize};

/// Creates the contact (system of record) and mirrors it as a platform user
/// (status defaults to invited).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactCreateRequest {
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "first_name", default)]
    pub first_name: String,
    /// The primary contact of its organization.
    #[serde(rename = "is_primary", default)]
    pub is_primary: bool,
    #[serde(rename = "last_name", default)]
    pub last_name: String,
    /// BCP 47, e.g. de-DE
    #[serde(rename = "locale", default)]
    pub locale: String,
    /// Owning organization — membership is mirrored to the platform team.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    #[serde(rename = "phone", default)]
    pub phone: String,
    /// Default 'buyer' — also the team role on the platform mirror.
    #[serde(rename = "role", default)]
    pub role: String,
    /// Default 'invited' on create.
    #[serde(rename = "status", default)]
    pub status: String,
}
