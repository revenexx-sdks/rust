use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceList {
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "currency", default)]
    pub currency: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    #[serde(rename = "priority", default)]
    pub priority: i64,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "tax_included", default)]
    pub tax_included: bool,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    #[serde(rename = "valid_from", default)]
    pub valid_from: String,
    #[serde(rename = "valid_until", default)]
    pub valid_until: String,
}
