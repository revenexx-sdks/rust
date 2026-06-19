use serde::{Deserialize, Serialize};

/// Topic list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopicList {
    /// List of topics.
    #[serde(rename = "topics", default)]
    pub topics: Vec<crate::models::Topic>,
    /// Total number of topics that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
