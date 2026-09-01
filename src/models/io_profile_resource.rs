use serde::{Deserialize, Serialize};

/// A saved profile. Mirrors the controller's presenter exactly — there
/// are no `created_at` / `updated_at` fields on this resource.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IoProfileResource {
    #[serde(rename = "app", default)]
    pub app: String,
    #[serde(rename = "apply_mode", default)]
    pub apply_mode: String,
    #[serde(rename = "created_by", default)]
    pub created_by: String,
    #[serde(rename = "direction", default)]
    pub direction: String,
    #[serde(rename = "entity", default)]
    pub entity: String,
    #[serde(rename = "format", default)]
    pub format: crate::models::IoProfileFormat,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "mapping", default)]
    pub mapping: serde_json::Value,
    /// `null` means global — offered for every market.
    #[serde(rename = "markets", default)]
    pub markets: Vec<String>,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "options", default)]
    pub options: serde_json::Value,
    #[serde(rename = "vendor", default)]
    pub vendor: String,
}
