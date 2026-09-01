use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditEntry {
    #[serde(rename = "action", default)]
    pub action: String,
    #[serde(rename = "changes", default)]
    pub changes: Vec<serde_json::Value>,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "resource_id", default)]
    pub resource_id: String,
    #[serde(rename = "resource_key", default)]
    pub resource_key: String,
    #[serde(rename = "resource_type", default)]
    pub resource_type: String,
    #[serde(rename = "subject", default)]
    pub subject: String,
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
}
