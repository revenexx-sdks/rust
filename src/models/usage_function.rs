use serde::{Deserialize, Serialize};

/// UsageFunction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsageFunction {
    /// Aggregated number of function builds per period.
    #[serde(rename = "builds", default)]
    pub builds: Vec<crate::models::Metric>,
    /// Aggregated number of failed builds per period.
    #[serde(rename = "buildsFailed", default)]
    pub builds_failed: Vec<crate::models::Metric>,
    /// Total aggregated number of failed function builds.
    #[serde(rename = "buildsFailedTotal", default)]
    pub builds_failed_total: i64,
    /// Aggregated number of function builds mbSeconds per period.
    #[serde(rename = "buildsMbSeconds", default)]
    pub builds_mb_seconds: Vec<crate::models::Metric>,
    /// Total aggregated sum of function builds mbSeconds.
    #[serde(rename = "buildsMbSecondsTotal", default)]
    pub builds_mb_seconds_total: i64,
    /// Aggregated sum of function builds storage per period.
    #[serde(rename = "buildsStorage", default)]
    pub builds_storage: Vec<crate::models::Metric>,
    /// total aggregated sum of function builds storage.
    #[serde(rename = "buildsStorageTotal", default)]
    pub builds_storage_total: i64,
    /// Aggregated number of successful builds per period.
    #[serde(rename = "buildsSuccess", default)]
    pub builds_success: Vec<crate::models::Metric>,
    /// Total aggregated number of successful function builds.
    #[serde(rename = "buildsSuccessTotal", default)]
    pub builds_success_total: i64,
    /// Aggregated sum of function builds compute time per period.
    #[serde(rename = "buildsTime", default)]
    pub builds_time: Vec<crate::models::Metric>,
    /// Average builds compute time.
    #[serde(rename = "buildsTimeAverage", default)]
    pub builds_time_average: i64,
    /// Total aggregated sum of function builds compute time.
    #[serde(rename = "buildsTimeTotal", default)]
    pub builds_time_total: i64,
    /// Total aggregated number of function builds.
    #[serde(rename = "buildsTotal", default)]
    pub builds_total: i64,
    /// Aggregated number of function deployments per period.
    #[serde(rename = "deployments", default)]
    pub deployments: Vec<crate::models::Metric>,
    /// Aggregated number of  function deployments storage per period.
    #[serde(rename = "deploymentsStorage", default)]
    pub deployments_storage: Vec<crate::models::Metric>,
    /// Total aggregated sum of function deployments storage.
    #[serde(rename = "deploymentsStorageTotal", default)]
    pub deployments_storage_total: i64,
    /// Total aggregated number of function deployments.
    #[serde(rename = "deploymentsTotal", default)]
    pub deployments_total: i64,
    /// Aggregated number of function executions per period.
    #[serde(rename = "executions", default)]
    pub executions: Vec<crate::models::Metric>,
    /// Aggregated number of function mbSeconds per period.
    #[serde(rename = "executionsMbSeconds", default)]
    pub executions_mb_seconds: Vec<crate::models::Metric>,
    /// Total aggregated sum of function executions mbSeconds.
    #[serde(rename = "executionsMbSecondsTotal", default)]
    pub executions_mb_seconds_total: i64,
    /// Aggregated number of function executions compute time per period.
    #[serde(rename = "executionsTime", default)]
    pub executions_time: Vec<crate::models::Metric>,
    /// Total aggregated sum of function  executions compute time.
    #[serde(rename = "executionsTimeTotal", default)]
    pub executions_time_total: i64,
    /// Total  aggregated number of function executions.
    #[serde(rename = "executionsTotal", default)]
    pub executions_total: i64,
    /// The time range of the usage stats.
    #[serde(rename = "range", default)]
    pub range: String,
}
