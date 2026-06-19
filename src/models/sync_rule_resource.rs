use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SyncRuleResource {
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "last_run_at", default)]
    pub last_run_at: String,
    #[serde(rename = "options", default)]
    pub options: Vec<serde_json::Value>,
    #[serde(rename = "schedule", default)]
    pub schedule: String,
    #[serde(rename = "sftp_account_id", default)]
    pub sftp_account_id: String,
    #[serde(rename = "source_path", default)]
    pub source_path: String,
    #[serde(rename = "target_folder_id", default)]
    pub target_folder_id: String,
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
}
