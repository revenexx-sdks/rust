use serde::{Deserialize, Serialize};

/// Subscriber
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Subscriber {
    /// Subscriber creation time in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Subscriber ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Subscriber update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// The target provider type. Can be one of the following: `email`, `sms` or
    /// `push`.
    #[serde(rename = "providerType", default)]
    pub provider_type: String,
    /// Target.
    #[serde(rename = "target", default)]
    pub target: crate::models::Target,
    /// Target ID.
    #[serde(rename = "targetId", default)]
    pub target_id: String,
    /// Topic ID.
    #[serde(rename = "topicId", default)]
    pub topic_id: String,
    /// Topic ID.
    #[serde(rename = "userId", default)]
    pub user_id: String,
    /// User Name.
    #[serde(rename = "userName", default)]
    pub user_name: String,
}
