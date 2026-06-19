use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Organization {
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "external_team_id", default)]
    pub external_team_id: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "settings", default)]
    pub settings: serde_json::Value,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    #[serde(rename = "vat_id", default)]
    pub vat_id: String,
}
