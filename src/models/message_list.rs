use serde::{Deserialize, Serialize};

/// Message list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageList {
    /// List of messages.
    #[serde(rename = "messages", default)]
    pub messages: Vec<crate::models::Message>,
    /// Total number of messages that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
