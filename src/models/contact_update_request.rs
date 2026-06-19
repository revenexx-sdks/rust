use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value;
/// external_user_id is mirror-managed and ignored.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactUpdateRequest {
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
