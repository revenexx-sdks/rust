use serde::{Deserialize, Serialize};

/// Health Queue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthQueue {
    /// Amount of actions in the queue.
    #[serde(rename = "size", default)]
    pub size: i64,
}
