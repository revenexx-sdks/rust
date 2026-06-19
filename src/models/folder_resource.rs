use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FolderResource {
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "is_system", default)]
    pub is_system: bool,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "parent_id", default)]
    pub parent_id: String,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
