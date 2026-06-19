use serde::{Deserialize, Serialize};

/// Health Time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthTime {
    /// Difference of unix remote and local timestamps in milliseconds.
    #[serde(rename = "diff", default)]
    pub diff: i64,
    /// Current unix timestamp of local server where Appwrite runs.
    #[serde(rename = "localTime", default)]
    pub local_time: i64,
    /// Current unix timestamp on trustful remote server.
    #[serde(rename = "remoteTime", default)]
    pub remote_time: i64,
}
