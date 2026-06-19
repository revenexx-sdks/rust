use serde::{Deserialize, Serialize};

/// Message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Message {
    /// Message creation time in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Message ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Message update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Data of the message.
    #[serde(rename = "data", default)]
    pub data: serde_json::Value,
    /// The time when the message was delivered.
    #[serde(rename = "deliveredAt", default)]
    pub delivered_at: String,
    /// Number of recipients the message was delivered to.
    #[serde(rename = "deliveredTotal", default)]
    pub delivered_total: i64,
    /// Delivery errors if any.
    #[serde(rename = "deliveryErrors", default)]
    pub delivery_errors: Vec<String>,
    /// Message provider type.
    #[serde(rename = "providerType", default)]
    pub provider_type: String,
    /// The scheduled time for message.
    #[serde(rename = "scheduledAt", default)]
    pub scheduled_at: String,
    /// Status of delivery.
    #[serde(rename = "status", default)]
    pub status: String,
    /// Target IDs set as recipients.
    #[serde(rename = "targets", default)]
    pub targets: Vec<String>,
    /// Topic IDs set as recipients.
    #[serde(rename = "topics", default)]
    pub topics: Vec<String>,
    /// User IDs set as recipients.
    #[serde(rename = "users", default)]
    pub users: Vec<String>,
}
