use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SyncHistory {
    #[serde(rename = "bytes_synced", default)]
    pub bytes_synced: i64,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "duration_ms", default)]
    pub duration_ms: i64,
    #[serde(rename = "error", default)]
    pub error: String,
    #[serde(rename = "id", default)]
    pub id: i64,
    #[serde(rename = "rule_id", default)]
    pub rule_id: String,
    #[serde(rename = "run_id", default)]
    pub run_id: String,
    #[serde(rename = "source_path", default)]
    pub source_path: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "target_asset_id", default)]
    pub target_asset_id: String,
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
}
