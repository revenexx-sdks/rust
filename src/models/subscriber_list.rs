use serde::{Deserialize, Serialize};

/// Subscriber list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubscriberList {
    /// List of subscribers.
    #[serde(rename = "subscribers", default)]
    pub subscribers: Vec<crate::models::Subscriber>,
    /// Total number of subscribers that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
