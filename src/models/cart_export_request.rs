use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartExportRequest {
    /// Ad-hoc export format (only without profile_id).
    #[serde(rename = "format", default)]
    pub format: String,
    /// Export profile to run; ad-hoc JSON/CSV export when omitted.
    #[serde(rename = "profile_id", default)]
    pub profile_id: String,
}
