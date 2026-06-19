use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentProvider {
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "credentials", default)]
    pub credentials: serde_json::Value,
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "options", default)]
    pub options: serde_json::Value,
    #[serde(rename = "provider", default)]
    pub provider: String,
    #[serde(rename = "test_mode", default)]
    pub test_mode: bool,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
    #[serde(rename = "webhook_secret", default)]
    pub webhook_secret: String,
}
