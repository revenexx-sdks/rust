use serde::{Deserialize, Serialize};

/// UsageFunctions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsageFunctions {
    /// Aggregated number of functions build per period.
    #[serde(rename = "builds", default)]
    pub builds: Vec<crate::models::Metric>,
    /// Aggregated number of failed function builds per period.
    #[serde(rename = "buildsFailed", default)]
    pub builds_failed: Vec<crate::models::Metric>,
    /// Total aggregated number of failed function builds.
    #[serde(rename = "buildsFailedTotal", default)]
    pub builds_failed_total: i64,
    /// Aggregated sum of functions build mbSeconds per period.
    #[serde(rename = "buildsMbSeconds", default)]
    pub builds_mb_seconds: Vec<crate::models::Metric>,
    /// Total aggregated sum of functions build mbSeconds.
    #[serde(rename = "buildsMbSecondsTotal", default)]
    pub builds_mb_seconds_total: i64,
    /// Aggregated sum of functions build storage per period.
    #[serde(rename = "buildsStorage", default)]
    pub builds_storage: Vec<crate::models::Metric>,
    /// total aggregated sum of functions build storage.
    #[serde(rename = "buildsStorageTotal", default)]
    pub builds_storage_total: i64,
    /// Aggregated number of successful function builds per period.
    #[serde(rename = "buildsSuccess", default)]
    pub builds_success: Vec<crate::models::Metric>,
    /// Total aggregated number of successful function builds.
    #[serde(rename = "buildsSuccessTotal", default)]
    pub builds_success_total: i64,
    /// Aggregated sum of  functions build compute time per period.
    #[serde(rename = "buildsTime", default)]
    pub builds_time: Vec<crate::models::Metric>,
    /// Total aggregated sum of functions build compute time.
    #[serde(rename = "buildsTimeTotal", default)]
    pub builds_time_total: i64,
    /// Total aggregated number of functions build.
    #[serde(rename = "buildsTotal", default)]
    pub builds_total: i64,
    /// Aggregated number of functions deployment per period.
    #[serde(rename = "deployments", default)]
    pub deployments: Vec<crate::models::Metric>,
    /// Aggregated number of  functions deployment storage per period.
    #[serde(rename = "deploymentsStorage", default)]
    pub deployments_storage: Vec<crate::models::Metric>,
    /// Total aggregated sum of functions deployment storage.
    #[serde(rename = "deploymentsStorageTotal", default)]
    pub deployments_storage_total: i64,
    /// Total aggregated number of functions deployments.
    #[serde(rename = "deploymentsTotal", default)]
    pub deployments_total: i64,
    /// Aggregated number of  functions execution per period.
    #[serde(rename = "executions", default)]
    pub executions: Vec<crate::models::Metric>,
    /// Aggregated number of functions mbSeconds per period.
    #[serde(rename = "executionsMbSeconds", default)]
    pub executions_mb_seconds: Vec<crate::models::Metric>,
    /// Total aggregated sum of functions execution mbSeconds.
    #[serde(rename = "executionsMbSecondsTotal", default)]
    pub executions_mb_seconds_total: i64,
    /// Aggregated number of functions execution compute time per period.
    #[serde(rename = "executionsTime", default)]
    pub executions_time: Vec<crate::models::Metric>,
    /// Total aggregated sum of functions  execution compute time.
    #[serde(rename = "executionsTimeTotal", default)]
    pub executions_time_total: i64,
    /// Total  aggregated number of functions execution.
    #[serde(rename = "executionsTotal", default)]
    pub executions_total: i64,
    /// Aggregated number of functions per period.
    #[serde(rename = "functions", default)]
    pub functions: Vec<crate::models::Metric>,
    /// Total aggregated number of functions.
    #[serde(rename = "functionsTotal", default)]
    pub functions_total: i64,
    /// Time range of the usage stats.
    #[serde(rename = "range", default)]
    pub range: String,
}
