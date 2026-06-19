use serde::{Deserialize, Serialize};

/// Health Antivirus
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthAntivirus {
    /// Antivirus status. Possible values are: `disabled`, `offline`, `online`
    #[serde(rename = "status", default)]
    pub status: String,
    /// Antivirus version.
    #[serde(rename = "version", default)]
    pub version: String,
}
