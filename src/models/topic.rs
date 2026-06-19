use serde::{Deserialize, Serialize};

/// Topic
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Topic {
    /// Topic creation time in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Topic ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Topic update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Total count of email subscribers subscribed to the topic.
    #[serde(rename = "emailTotal", default)]
    pub email_total: i64,
    /// The name of the topic.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Total count of push subscribers subscribed to the topic.
    #[serde(rename = "pushTotal", default)]
    pub push_total: i64,
    /// Total count of SMS subscribers subscribed to the topic.
    #[serde(rename = "smsTotal", default)]
    pub sms_total: i64,
    /// Subscribe permissions.
    #[serde(rename = "subscribe", default)]
    pub subscribe: Vec<String>,
}
