use serde::{Deserialize, Serialize};

/// When this working copy should go live.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageScheduleRequest {
    /// The moment to publish at. Stored on the edit state and echoed back
    /// normalized to UTC.
    #[serde(rename = "scheduledAt", default)]
    pub scheduled_at: String,
}
