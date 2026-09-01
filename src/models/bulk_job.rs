use serde::{Deserialize, Serialize};

/// A bulk job as returned by `/bulk-jobs`. Note that the row counts are
/// nested under `counts` — they are not top-level fields — and that the
/// response carries no `tenant_id` (the listing envelope does) and no
/// `updated_at`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BulkJob {
    #[serde(rename = "app", default)]
    pub app: String,
    #[serde(rename = "correlation_id", default)]
    pub correlation_id: String,
    #[serde(rename = "counts", default)]
    pub counts: serde_json::Value,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "created_by", default)]
    pub created_by: String,
    #[serde(rename = "duration_ms", default)]
    pub duration_ms: i64,
    #[serde(rename = "entity", default)]
    pub entity: String,
    #[serde(rename = "error_message", default)]
    pub error_message: String,
    #[serde(rename = "finished_at", default)]
    pub finished_at: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "profile_id", default)]
    pub profile_id: String,
    /// Engine-reported progress. For an export this carries the
    /// `object_key` and `format` the result is written to.
    #[serde(rename = "progress", default)]
    pub progress: serde_json::Value,
    #[serde(rename = "started_at", default)]
    pub started_at: String,
    #[serde(rename = "status", default)]
    pub status: crate::models::BulkJobStatus,
    #[serde(rename = "type", default)]
    pub xtype: crate::models::BulkJobType,
    #[serde(rename = "vendor", default)]
    pub vendor: String,
}
