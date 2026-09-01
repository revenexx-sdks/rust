use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TenantConfig {
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    #[serde(rename = "default_locale", default)]
    pub default_locale: String,
    #[serde(rename = "defaults", default)]
    pub defaults: Vec<serde_json::Value>,
    #[serde(rename = "delivery_reporting", default)]
    pub delivery_reporting: Vec<serde_json::Value>,
    #[serde(rename = "locales", default)]
    pub locales: Vec<serde_json::Value>,
    #[serde(rename = "product", default)]
    pub product: String,
    #[serde(rename = "provisioned_at", default)]
    pub provisioned_at: String,
    #[serde(rename = "quiet_hours", default)]
    pub quiet_hours: Vec<serde_json::Value>,
    #[serde(rename = "quotas", default)]
    pub quotas: Vec<serde_json::Value>,
    #[serde(rename = "retention_days", default)]
    pub retention_days: i64,
    #[serde(rename = "support_email", default)]
    pub support_email: String,
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
