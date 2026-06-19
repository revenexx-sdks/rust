use serde::{Deserialize, Serialize};

/// Provider
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Provider {
    /// Provider creation time in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Provider ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Provider update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Provider credentials.
    #[serde(rename = "credentials", default)]
    pub credentials: serde_json::Value,
    /// Is provider enabled?
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// The name for the provider instance.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Provider options.
    #[serde(rename = "options", default)]
    pub options: serde_json::Value,
    /// The name of the provider service.
    #[serde(rename = "provider", default)]
    pub provider: String,
    /// Type of provider.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
