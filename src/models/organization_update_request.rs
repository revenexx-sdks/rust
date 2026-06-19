use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value;
/// external_team_id is mirror-managed and ignored.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrganizationUpdateRequest {
    /// Company name — mirrored to the platform team.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Free-form organization settings.
    #[serde(rename = "settings", default)]
    pub settings: serde_json::Value,
    /// Default 'active'.
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "vat_id", default)]
    pub vat_id: String,
}
